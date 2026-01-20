# Référentiel Général de Sécurité version 2.0

## Errata de l'annexe A3

### Politique de Certification Type « certificats électroniques de services applicatifs »

**Version 1.0 du 19 octobre 2016**

---

## Historique des versions

| Date | Version | Évolution du document | Rédacteur |
|------|---------|----------------------|-----------|
| 19/10/2016 | 1.0 | Version pour application au 19 octobre 2016 | ANSSI |

Les commentaires sur le présent document sont à adresser à :

> Agence nationale de la sécurité des systèmes d'information
> SGDSN/ANSSI
> 51 boulevard de La Tour-Maubourg
> 75700 Paris 07 SP
> rgs@ssi.gouv.fr

---

## Sommaire

- [I. Introduction](#i-introduction)
  - [I.1. Présentation générale](#i1-présentation-générale)
  - [I.2. Identification du document](#i2-identification-du-document)
  - [I.3. Acronymes](#i3-acronymes)
- [II. Errata](#ii-errata)
- [III. Annexe : Documents cités en référence](#iii-annexe--documents-cités-en-référence)

---

## I. Introduction

### I.1. Présentation générale

Le document « Politique de Certification Type, certificats électroniques de services applicatifs » (PC Type Services applicatifs) fait partie du Référentiel Général de Sécurité [RGS]. Il en constitue l'annexe [RGS_A3].

Le présent document constitue l'errata de l'annexe [RGS_A3].

### I.2. Identification du document

Le présent errata est dénommée "RGS – Errata de la Politique de Certification Type – certificats électroniques de services applicatifs". Il peut être identifié par son nom, numéro de version et sa date de mise à jour.

### I.3. Acronymes

Les acronymes utilisés dans le présent document sont les suivants :

| Acronyme | Définition |
|----------|------------|
| AC | Autorité de Certification |
| ANSSI | Agence nationale de la sécurité des systèmes d'information |
| PC | Politique de Certification |

---

## II. Errata

Dans le chapitre **VI.3.2 Durées de vie des bi-clés et des certificats** de l'annexe [RGS_A3], le texte :

> « Les bi-clés et les certificats des services applicatifs couverts par la présente PC Type doivent avoir une durée de vie maximale de 3 ans. »

est complété par le texte suivant :

> « Pour les certificats cachet dont la clé privée est utilisée pour signer des contremarques de temps, la durée de vie maximale est à la place bornée selon le couple {durée de vie cryptographique de la clé ; fin de validité du certificat de l'AC émettrice}. »

---

## III. Annexe : Documents cités en référence

| Renvoi | Document |
|--------|----------|
| [RGS] | *Référentiel Général de Sécurité – Version 2.0* |
| [RGS_A3] | *RGS - Politique de Certification Type – Certificats électroniques de services applicatifs – Version 3.0* |
