---
title: "Exemple : exigences de sécurité pour une plateforme e-commerce"
template_version: "1.0.0"
constitutional_principle: "III - sécurité dès la conception"
project: "ShopSecure - Plateforme e-commerce"
ssdlc_phase: "design"
tags: ["example", "ecommerce", "security-requirements", "owasp-asvs"]
---

# Exemple : exigences de sécurité pour ShopSecure

**Projet** : ShopSecure - Plateforme e-commerce B2C
**Contexte** : application web permettant aux clients d'acheter des produits en ligne
**Stack technique** : React (frontend), Node.js/Express (backend), PostgreSQL, Stripe (paiement)

---

## 1. Exigences d'authentification

### REQ-AUTH-001 : Politique de mots de passe forte

**Exigence** : Les mots de passe doivent respecter les critères suivants :
- Minimum 12 caractères
- Au moins 1 majuscule, 1 minuscule, 1 chiffre, 1 caractère spécial
- Pas de mots du dictionnaire
- Différent des 5 derniers mots de passe

**OWASP ASVS** : V2.1.1 (Level 2)

**Implémentation** :
```javascript
// backend/validators/password.js
const passwordPolicy = {
  minLength: 12,
  minLowercase: 1,
  minUppercase: 1,
  minNumbers: 1,
  minSymbols: 1,
  returnScore: false,
  pointsPerUnique: 1,
  pointsPerRepeat: 0.5,
  pointsForContainingLower: 10,
  pointsForContainingUpper: 10,
  pointsForContainingNumber: 10,
  pointsForContainingSymbol: 10
};

function validatePassword(password, userHistory) {
  // Vérifier longueur
  if (password.length < 12) {
    return { valid: false, error: 'Minimum 12 caractères requis' };
  }

  // Vérifier complexité
  const hasUpper = /[A-Z]/.test(password);
  const hasLower = /[a-z]/.test(password);
  const hasNumber = /[0-9]/.test(password);
  const hasSpecial = /[!@#$%^&*(),.?":{}|<>]/.test(password);

  if (!hasUpper || !hasLower || !hasNumber || !hasSpecial) {
    return { valid: false, error: 'Complexité insuffisante' };
  }

  // Vérifier historique (5 derniers mots de passe)
  const bcrypt = require('bcrypt');
  for (const oldHash of userHistory.slice(-5)) {
    if (bcrypt.compareSync(password, oldHash)) {
      return { valid: false, error: 'Mot de passe déjà utilisé récemment' };
    }
  }

  return { valid: true };
}
```

**Tests** :
```javascript
describe('Password Policy', () => {
  test('should reject password < 12 chars', () => {
    expect(validatePassword('Short1!')).toEqual({
      valid: false,
      error: 'Minimum 12 caractères requis'
    });
  });

  test('should accept valid strong password', () => {
    expect(validatePassword('MySecureP@ssw0rd2024')).toEqual({
      valid: true
    });
  });
});
```

---

### REQ-AUTH-002 : Authentification multi-facteurs (MFA)

**Exigence** : MFA obligatoire pour :
- Tous les comptes administrateurs
- Comptes clients avec > 500€ de transactions cumulées
- Optionnel pour autres comptes clients

**OWASP ASVS** : V2.8.1 (Level 2)

**Implémentation** :
```javascript
// backend/middleware/mfa.js
async function requireMFA(req, res, next) {
  const user = await User.findById(req.userId);

  // MFA obligatoire pour admins
  if (user.role === 'admin' && !req.session.mfaVerified) {
    return res.status(403).json({
      error: 'MFA required',
      redirectTo: '/auth/mfa-challenge'
    });
  }

  // MFA obligatoire si transactions > 500€
  const totalSpent = await Order.sum('amount', { where: { userId: user.id }});
  if (totalSpent > 500 && !req.session.mfaVerified) {
    return res.status(403).json({
      error: 'MFA required for high-value accounts',
      redirectTo: '/auth/mfa-challenge'
    });
  }

  next();
}

// Génération du QR code TOTP
const speakeasy = require('speakeasy');
const QRCode = require('qrcode');

router.post('/auth/mfa/setup', async (req, res) => {
  const secret = speakeasy.generateSecret({
    name: `ShopSecure (${req.user.email})`
  });

  await User.update(
    { mfaSecret: secret.base32 },
    { where: { id: req.userId }}
  );

  const qrCodeUrl = await QRCode.toDataURL(secret.otpauth_url);

  res.json({ qrCode: qrCodeUrl, secret: secret.base32 });
});

// Vérification du code TOTP
router.post('/auth/mfa/verify', async (req, res) => {
  const user = await User.findById(req.userId);

  const verified = speakeasy.totp.verify({
    secret: user.mfaSecret,
    encoding: 'base32',
    token: req.body.code,
    window: 2  // ±2 intervalles (30s chacun)
  });

  if (verified) {
    req.session.mfaVerified = true;
    res.json({ success: true });
  } else {
    res.status(401).json({ error: 'Code invalide' });
  }
});
```

---

## 2. Exigences d'autorisation

### REQ-AUTHZ-001 : Contrôle d'accès basé sur les rôles (RBAC)

**Exigence** : Implémenter 3 rôles avec permissions distinctes :

| Rôle | Permissions |
|------|-------------|
| **Customer** | Voir produits, passer commandes, voir ses propres commandes |
| **Support** | Voir toutes les commandes, modifier statuts, contacter clients |
| **Admin** | Toutes permissions + gestion utilisateurs + configuration |

**OWASP ASVS** : V4.1.1 (Level 1)

**Implémentation** :
```javascript
// backend/middleware/rbac.js
const permissions = {
  customer: ['products:read', 'orders:create', 'orders:read:own'],
  support: ['products:read', 'orders:read:all', 'orders:update', 'customers:contact'],
  admin: ['*']  // Toutes permissions
};

function can(role, permission, resource) {
  const rolePerms = permissions[role] || [];

  // Admin a tous les droits
  if (rolePerms.includes('*')) return true;

  // Vérifier permission exacte
  if (rolePerms.includes(permission)) {
    // Si permission "own", vérifier ownership
    if (permission.endsWith(':own')) {
      return resource.userId === req.userId;
    }
    return true;
  }

  return false;
}

// Middleware
function requirePermission(permission) {
  return async (req, res, next) => {
    const user = await User.findById(req.userId);

    if (!can(user.role, permission, req.params)) {
      return res.status(403).json({
        error: 'Insufficient permissions',
        required: permission,
        userRole: user.role
      });
    }

    next();
  };
}

// Utilisation
router.get('/orders/:id',
  authenticate,
  requirePermission('orders:read:own'),
  async (req, res) => {
    const order = await Order.findById(req.params.id);

    // Vérifier ownership si pas admin/support
    if (req.user.role === 'customer' && order.userId !== req.userId) {
      return res.status(403).json({ error: 'Access denied' });
    }

    res.json(order);
  }
);
```

---

### REQ-AUTHZ-002 : Protection contre IDOR

**Exigence** : Empêcher l'accès aux ressources d'autres utilisateurs

**OWASP ASVS** : V4.1.2 (Level 1)

**Implémentation** :
```javascript
// backend/routes/orders.js
router.get('/orders/:id', authenticate, async (req, res) => {
  const order = await Order.findOne({
    where: {
      id: req.params.id,
      userId: req.userId  // ← Filtrage automatique par userId
    }
  });

  if (!order) {
    // Ne pas révéler si l'ID existe mais appartient à quelqu'un d'autre
    return res.status(404).json({ error: 'Order not found' });
  }

  res.json(order);
});

// Mauvaise pratique ❌
router.get('/orders/:id', authenticate, async (req, res) => {
  const order = await Order.findById(req.params.id);

  // Check après fetch = vulnérable aux timing attacks
  if (order.userId !== req.userId) {
    return res.status(403).json({ error: 'Access denied' });
  }

  res.json(order);
});
```

---

## 3. Exigences de chiffrement

### REQ-CRYPTO-001 : Chiffrement des données sensibles au repos

**Exigence** : Chiffrer les données suivantes en base de données :
- Numéros de carte de crédit (derniers 4 chiffres uniquement stockés)
- Adresses complètes des clients
- Numéros de téléphone

**OWASP ASVS** : V6.2.1 (Level 2)

**Implémentation** :
```javascript
// backend/utils/encryption.js
const crypto = require('crypto');

const algorithm = 'aes-256-gcm';
const key = Buffer.from(process.env.ENCRYPTION_KEY, 'hex'); // 32 bytes

function encrypt(plaintext) {
  const iv = crypto.randomBytes(16);
  const cipher = crypto.createCipheriv(algorithm, key, iv);

  let encrypted = cipher.update(plaintext, 'utf8', 'hex');
  encrypted += cipher.final('hex');

  const authTag = cipher.getAuthTag();

  return {
    iv: iv.toString('hex'),
    encryptedData: encrypted,
    authTag: authTag.toString('hex')
  };
}

function decrypt(encryptedObj) {
  const decipher = crypto.createDecipheriv(
    algorithm,
    key,
    Buffer.from(encryptedObj.iv, 'hex')
  );

  decipher.setAuthTag(Buffer.from(encryptedObj.authTag, 'hex'));

  let decrypted = decipher.update(encryptedObj.encryptedData, 'hex', 'utf8');
  decrypted += decipher.final('utf8');

  return decrypted;
}

// Model User
class User extends Model {
  static init(sequelize) {
    super.init({
      email: DataTypes.STRING,
      phone: {
        type: DataTypes.TEXT,
        get() {
          const encrypted = this.getDataValue('phone');
          if (!encrypted) return null;
          return decrypt(JSON.parse(encrypted));
        },
        set(value) {
          if (!value) return;
          const encrypted = encrypt(value);
          this.setDataValue('phone', JSON.stringify(encrypted));
        }
      },
      address: {
        type: DataTypes.TEXT,
        get() {
          const encrypted = this.getDataValue('address');
          if (!encrypted) return null;
          return decrypt(JSON.parse(encrypted));
        },
        set(value) {
          if (!value) return;
          const encrypted = encrypt(value);
          this.setDataValue('address', JSON.stringify(encrypted));
        }
      }
    }, { sequelize });
  }
}
```

---

### REQ-CRYPTO-002 : TLS 1.3 obligatoire

**Exigence** : Toutes les communications HTTP doivent utiliser TLS 1.3 minimum

**OWASP ASVS** : V9.1.1 (Level 1)

**Implémentation** :
```nginx
# nginx.conf
server {
    listen 443 ssl http2;
    server_name shopsecure.com;

    # TLS 1.3 uniquement
    ssl_protocols TLSv1.3;

    # Certificat
    ssl_certificate /etc/nginx/ssl/cert.pem;
    ssl_certificate_key /etc/nginx/ssl/key.pem;

    # Ciphers modernes
    ssl_ciphers 'TLS_AES_256_GCM_SHA384:TLS_CHACHA20_POLY1305_SHA256:TLS_AES_128_GCM_SHA256';
    ssl_prefer_server_ciphers off;

    # HSTS
    add_header Strict-Transport-Security "max-age=63072000; includeSubDomains; preload" always;

    # Rediriger HTTP → HTTPS
    if ($scheme != "https") {
        return 301 https://$server_name$request_uri;
    }
}
```

---

## 4. Exigences de validation des entrées

### REQ-INPUT-001 : Validation et sanitization

**Exigence** : Valider et nettoyer toutes les entrées utilisateur

**OWASP ASVS** : V5.1.1 (Level 1)

**Implémentation** :
```javascript
// backend/validators/product.js
const { body, param, validationResult } = require('express-validator');
const xss = require('xss');

const createProductValidator = [
  body('name')
    .trim()
    .isLength({ min: 3, max: 100 })
    .customSanitizer(xss)
    .withMessage('Nom produit invalide'),

  body('price')
    .isFloat({ min: 0.01, max: 999999 })
    .withMessage('Prix invalide'),

  body('description')
    .optional()
    .trim()
    .isLength({ max: 5000 })
    .customSanitizer(xss),

  body('category')
    .isIn(['electronics', 'clothing', 'books', 'home'])
    .withMessage('Catégorie invalide'),

  (req, res, next) => {
    const errors = validationResult(req);
    if (!errors.isEmpty()) {
      return res.status(400).json({ errors: errors.array() });
    }
    next();
  }
];

router.post('/products',
  authenticate,
  requirePermission('products:create'),
  createProductValidator,
  async (req, res) => {
    const product = await Product.create(req.body);
    res.status(201).json(product);
  }
);
```

---

## 5. Headers de sécurité HTTP

### REQ-HTTP-001 : Headers de sécurité obligatoires

**Exigence** : Implémenter tous les headers de sécurité HTTP

**OWASP ASVS** : V14.4.1-7 (Level 1)

**Implémentation** :
```javascript
// backend/middleware/security-headers.js
const helmet = require('helmet');

app.use(helmet({
  contentSecurityPolicy: {
    directives: {
      defaultSrc: ["'self'"],
      scriptSrc: ["'self'", "'unsafe-inline'", "https://cdn.stripe.com"],
      styleSrc: ["'self'", "'unsafe-inline'"],
      imgSrc: ["'self'", "data:", "https:"],
      connectSrc: ["'self'", "https://api.stripe.com"],
      fontSrc: ["'self'"],
      objectSrc: ["'none'"],
      mediaSrc: ["'self'"],
      frameSrc: ["https://js.stripe.com"],
      upgradeInsecureRequests: []
    }
  },
  hsts: {
    maxAge: 63072000,
    includeSubDomains: true,
    preload: true
  },
  frameguard: { action: 'deny' },
  noSniff: true,
  xssFilter: true,
  referrerPolicy: { policy: 'no-referrer' }
}));

// Retirer headers révélant la stack
app.disable('x-powered-by');
```

---

## 6. Checklist de validation OWASP ASVS

### Niveau 1 (Minimum pour production)

- [x] **V1.2** : Politique de mots de passe forte
- [x] **V4.1** : RBAC implémenté
- [x] **V4.1** : Protection IDOR
- [x] **V5.1** : Validation des entrées
- [x] **V6.2** : Chiffrement données sensibles au repos
- [x] **V9.1** : TLS 1.3 obligatoire
- [x] **V14.4** : Headers de sécurité HTTP

### Niveau 2 (Recommandé)

- [x] **V2.8** : MFA pour comptes sensibles
- [x] **V5.2** : Sanitization XSS
- [x] **V6.2** : Chiffrement avec AES-256-GCM

### Niveau 3 (Pour applications critiques)

- [ ] **V2.2** : Authentification hardware (YubiKey)
- [ ] **V6.3** : HSM pour gestion des clés
- [ ] **V8.3** : Protection contre la désérialisation

---

## 7. Métriques de conformité

| Exigence | Statut | Couverture tests | Notes |
|----------|--------|------------------|-------|
| REQ-AUTH-001 | ✅ Implémenté | 95% | Tests unitaires + intégration |
| REQ-AUTH-002 | ✅ Implémenté | 85% | MFA optionnel pour clients standards |
| REQ-AUTHZ-001 | ✅ Implémenté | 90% | 3 rôles couverts |
| REQ-AUTHZ-002 | ✅ Implémenté | 100% | Tests IDOR complets |
| REQ-CRYPTO-001 | ✅ Implémenté | 80% | Chiffrement phone + address |
| REQ-CRYPTO-002 | ✅ Implémenté | N/A | Nginx config |
| REQ-INPUT-001 | ✅ Implémenté | 90% | Tous endpoints validés |
| REQ-HTTP-001 | ✅ Implémenté | 100% | Helmet configuré |

**Couverture globale** : **91%**

---

**Prochaine étape** : Implémenter les tests de sécurité (Principe IV)
