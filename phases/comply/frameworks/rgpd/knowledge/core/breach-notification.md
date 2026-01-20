# Guidelines 9/2022 on personal data breach notification under GDPR

> Source: https://www.edpb.europa.eu/system/files/2023-04/edpb_guidelines_202209_personal_data_breach_notification_v2.0_en.pdf
> Version: 2.0
> Language: EN

---

Guidelines 9/2022 on personal data breach notification
                    under GDPR

                      Version 2.0

                Adopted 28 March 2023
Version history
 Version 1.0   10 October 2022   Adoption of the Guidelines (updated version of the previous
                                 guidelines WP250 (rev.01) adopted by the Working Party 29
                                 and endorsed by the EDPB on 25 May 2018) for a targeted
                                 public consultation.

 Version 2.0   28 March 2023     Adoption of the Guidelines following the targeted public
                                 consultation on the subject of data breach notification for
                                 controllers not established in the EEA.




Adopted                                                                                    2
TABLE OF CONTENTS
0          PREFACE ............................................................................................................................... 5

INTRODUCTION ............................................................................................................................ 5

I.         PERSONAL DATA BREACH NOTIFICATION UNDER THE GDPR ................................................... 7

      A.      Basic security considerations ..................................................................................................... 7

      B.      What is a personal data breach? ................................................................................................ 7

           1. Definition ................................................................................................................................... 7

           2. Types of personal data breaches ............................................................................................... 8

           3. The possible consequences of a personal data breach ............................................................. 8

      C.      Notification to the supervisory authority ................................................................................. 10

           1. When to notify ......................................................................................................................... 10
               a)     Article 33 - Basic requirements........................................................................................ 10
               b)     When does a controller become "aware"? ..................................................................... 11
               c)     Joint Controllers .............................................................................................................. 14
               d)     Controller – Processor obligations .................................................................................. 14
               e)     Which data breaches must be notified to the supervisory authority?............................ 16
               f)     Summary ......................................................................................................................... 19

           2. Providing information to the supervisory authority ............................................................... 20
               a)     Information to be provided ............................................................................................. 20
               b)     Notification in phases ...................................................................................................... 21

           3. Delayed notifications ............................................................................................................... 22

           4. Cross-border breaches and breaches at non-EU establishments ............................................ 23
               a)   To which supervisory authority (or authorities) should the notification be made if there
               is a cross-border personal data breach? ................................................................................. 23
               b)  Breaches at non-EU establishments .............................................................................. 26

      D.      Communication to the data subject ......................................................................................... 27

           1. Informing individuals ............................................................................................................... 27

           2. Information to be provided ..................................................................................................... 28

           3. How should individuals be contacted? .................................................................................... 29

           4. Conditions when communication is not required ................................................................... 30

      E.      Documenting breaches ............................................................................................................. 31

      F.      Risk assessment and examples of various breach scenarios .................................................... 32

           1. Risk assessment ....................................................................................................................... 32

           2. Conditions for risk ................................................................................................................... 32

           3. Categories of data breaches .................................................................................................... 34



Adopted                                                                                                                                               3
              a)   Ransomware attacks ....................................................................................................... 34
              b)   Data exfiltration attacks .................................................................................................. 35
              c)   Internal human risk source ............................................................................................. 35
              d)   Lost or stolen devices and paper documents ................................................................. 36
              e)   Mis-delivery ..................................................................................................................... 36
              f)   Social engineering ........................................................................................................... 36
              g)   Other ............................................................................................................................... 36

II.        BREACH NOTIFICATION OBLIGATIONS UNDER OTHER LEGAL INSTRUMENTS AND
NOTIFICATION TO OTHER ORGANISATIONS ...................................................................................... 37

      A.      Security breach notification requirements for providers of electronic communication services
              37

      B.      Breach notification requirements for trust service providers .................................................. 38

      C.      Other notification obligations ................................................................................................... 38

      D.      Other notifications .................................................................................................................... 39

ANNEX A - FLOWCHART SHOWING NOTIFICATION REQUIREMENTS ................................................... 40

ANNEX B - CASE STUDY EXAMPLES FROM A NOTIFICATION PERSPECTIVE ............................................ 41

      Example 1: A ransomware attack with backup available and no exfiltration in a hospital .............. 42

      Example 2: A ransomware attack with no backup or exfiltration of personal data .......................... 43

      Example 3: A data exfiltration attack on a job application website .................................................. 44

      Example 4: An exfiltration of hashed passwords from a website ..................................................... 44

      Example 5: A successful spear phishing attack .................................................................................. 45

      Example 6: Theft of a portable device storing encrypted personal data .......................................... 46

      Example 7: Theft of paper files with sensitive data ........................................................................... 47

      Example 8: Customer credentials stolen from a bank website ......................................................... 48

      Example 9: A mis-delivery of postal mail ........................................................................................... 48

      Example 10: A mis-delivery of highly confidential data ..................................................................... 49

      Example 11: A personal data breach affecting only employees ........................................................ 49

      Example 12: Identity theft ................................................................................................................. 50

      Example 13: Email exfiltration ........................................................................................................... 50

      Example 14: A theft of business documents by a former employee ................................................. 51

      Example 15: An accidental upload of personal data to a social media platform by an employee .... 51

      Example 16: Encryption by ransomware without exfiltration of a transport company .................... 52

      Example 17: An exfiltration of business data by a former employee ................................................ 53

      Example 18: A successful exfiltration of exam answers from a university ........................................ 54



Adopted                                                                                                                                                4
0          PREFACE
These Guidelines explain the mandatory personal data breach notification and communication
requirements of the GDPR. They explain when a personal data breach must be notified to the competent
supervisory authority and the data subjects, what information must be provided as part of the notification
and communication, as well as some guidance on the actions that a controller should take for data breach
prevention. It also contains a series of examples that illustrate how a variety of breaches should be
assessed against the notification thresholds. The annexes contain a flowchart summarising the notification
requirements, and a set of good/bad practice examples taking the controller through a complete breach
from initial assessment to taking remedial measures.

In line with the provisions of the GDPR, the original breach notification guidelines WP250 were adopted
by the Article 29 Working Party (WP29) on 3 October 2017 and, after revision, finally on 6 February 2018.
The WP29 was replaced by the EDPB on 25 May 2018 and WP250 was endorsed by the EDPB at that time.

Following the ruling of the Court of Justice of the European Union (CJEU) in Schrems II, the EDPB
undertook an analysis of notification requirements for cross-border breaches, particularly in cases where
the controller is established outside of the EEA. The EDPB considered it was necessary to amend WP250
to take account of the ruling and provide further guidance for the handling of cross-border breaches in
such scenarios. A short targeted public consultation was published on 19 October 2022. The Guidelines
were adopted following public consultation on 28 March 2023.

The Guidelines are provided for general guidance purposes only and should not be used as a substitute
for seeking legal advice. It should be emphasised that the GDPR sets out minimum standards that may
lead to higher thresholds where special circumstances warrant. Specific guidance on special categories of
personal data, and processing operations, that are high risk is not addressed and should be considered
carefully as part of the overall assessment.

The Guidelines will be reviewed by the EDPB periodically, and updated as necessary.


INTRODUCTION
The General Data Protection Regulation1 (GDPR) introduces the requirement for a personal data breach
(hereafter referred to as a "breach") to be notified to the competent national supervisory authority2
(hereafter referred to as a "supervisory authority" or "SA") and, in certain cases, to communicate the
breach to the individuals whose personal data have been affected by the breach.

The obligation to report breaches already exists for certain organisations, such as providers of publicly
available electronic communications services (as specified in Directive 2009/136/EC and Regulation (EU)
No 611/2013).3 There are also some Member States that already have their own national breach

1 Regulation (EU) 2016/679 of the European Parliament and of the Council of 27 April 2016 on the protection of
natural persons with regard to the processing of personal data and on the free movement of such data, and
repealing Directive 95/46/EC (General Data Protection Regulation).
2
  See Article 4(21) and Article 51 GDPR.
3
  Directive 2002/58/EC of the European Parliament and of the Council of 12 July 2002 concerning the processing of
personal data and the protection of privacy in the electronic communications sector (Directive on privacy and

Adopted                                                                                                         5
notification obligation. This may have included the obligation to notify data breaches to a Data Protection
Authority or other body (e.g. CERT for certain breaches in the IT sector), as well as an obligation to notify
affected individuals.4 While some Member States had previously published guidance on breach
notification,5 the GDPR now introduces an obligation for all controllers to notify the competent SA, and
in certain cases affected data subjects of a breach. Processors also have an important role to play and
they must notify any breach to their controller.6

This breach notification obligation under the GDPR applies to the processing of personal data by all data
controllers, regardless of the scope of the processing, and not only to providers of publicly available
electronic communications services. 7

These Guidelines first explain the breach notification requirements under the GDPR and then describe
some practical steps controllers can take to comply. The Annexes at the end of these Guidelines provide
a visual summary of the breach notification requirements, and case study examples based on actual data
breach cases notified to supervisory authorities.




electronic communications), as amended by Directive 2009/136/EC of the European Parliament and of the Council
of 25 November 2009 ("ePrivacy Directive"). Commission Regulation (EU) No 611/2013 of 24 June 2013 on the
measures applicable to the notification of personal data breaches under Directive 2002/58/EC of the European
Parliament and of the Council on privacy and electronic communications.
4
  For example, the German Federal Data Protection Act (Bundesdatenschutzgesetz) required controllers to notify
in the case of certain categories of data breaches – Section 42a
(https://www.gesetze-im-internet.de/englisch_bdsg/englisch_bdsg.html#p0333). Italy had similar breach
notification obligations under the Data Protection Authority Order of 2 July 2015, in addition to those relating to
electronic communication services (see Section V below). In the Netherlands, a general breach notification
obligation was introduced by legislation on 1 January 2016 (see Article 34a of the Dutch Data Protection Act
(https://autoriteitpersoonsgegevens.nl/sites/default/files/atoms/files/wbp_unofficial_translation_october_2016.p
df).
5
  For example, see Dutch Data Protection Authority: Beleidsregels meldplicht datalekken (in Dutch only),
https://autoriteitpersoonsgegevens.nl/sites/default/files/atoms/files/beleidsregels_meldplicht_datalekken.pdf.
6
  See Article 28(3)(f) and Article 33(2) GDPR.
7
  Some Member States may be required by local law to report certain types of breaches to other bodies, such as
law enforcement. This is outside the scope of these Guidelines, which focus solely on breach notification under the
GDPR.

Adopted                                                                                                           6
I.         PERSONAL DATA BREACH NOTIFICATION UNDER THE
           GDPR

      A.      Basic security considerations
Article 32 requires controllers and processors to implement appropriate technical and organisational
measures to ensure a level of security appropriate to the risk posed by their processing. These measures
should include, as appropriate:

    a) the pseudonymisation and encryption of personal data;
    b) the ability to ensure the ongoing confidentiality, integrity, availability and resilience of processing
       systems and services;
    c) the ability to restore the availability and access to personal data in a timely manner in the event
       of a physical or technical incident;
    d) a process for regularly testing, assessing and evaluating the effectiveness of technical and
       organisational measures for ensuring the security of the processing.

An important aspect of any data security policy is being able, where possible, to prevent a breach and,
where it nevertheless occurs, to react to it in a timely manner.

To this end, Recital 87 explains:

"It should be ascertained whether all appropriate technological protection and organisational measures
have been implemented to establish immediately whether a personal data breach has taken place and to
inform promptly the supervisory authority and the data subject. The fact that the notification was made
without undue delay should be established taking into account in particular the nature and gravity of the
personal data breach and its consequences and adverse effects for the data subject. Such notification may
result in an intervention of the supervisory authority in accordance with its tasks and powers laid down
in this Regulation."

Thus, both Articles 33 and 34 must be considered alongside the general security requirement in Article
32.


      B.      What is a personal data breach?

           1. Definition
"Personal data breach" is defined in Article 4(12) as:

"a breach of security leading to the accidental or unlawful destruction, loss, alteration, unauthorised
disclosure of, or access to, personal data transmitted, stored or otherwise processed".

The definition makes clear that a breach is a type of security incident. However, as indicated by the use
of the phrase, "leading to", the GDPR only applies where the incident results in personal data being
compromised. A security incident that does not result in personal data being compromised is not a
personal data breach within the meaning of the GDPR and therefore is not subject to the GDPR
notification requirements.

The consequences of a breach can be significant for individuals: they can suffer a variety of harms such
as physical, material, or non-material damage. The GDPR explains that this can include loss of control

Adopted                                                                                                    7
over their personal data, limitation of their rights, discrimination, identity theft or fraud, financial loss,
unauthorised reversal of pseudonymisation, damage to reputation, and loss of confidentiality of personal
data protected by professional secrecy. It can also include any other significant economic or social
disadvantage to those individuals.8 Therefore, the GDPR requires the controller to notify a breach to the
competent supervisory authority, unless it is unlikely to result in a risk to the rights and freedoms of
individuals, and to communicate a breach to affected individuals if it is likely to result in a high risk to their
rights and freedoms. This is part of the overall GDPR requirements to implement appropriate technical
and organisational measures to protect personal data and to demonstrate compliance.


          2. Types of personal data breaches
Breaches can be categorised according to the following three well-known information security
principles:9

      •    "Confidentiality breach" - where there is an unauthorised or accidental disclosure of, or access
           to, personal data.
      •    "Integrity breach" - where there is an unauthorised or accidental alteration of personal data.
      •    "Availability breach" - where there is an accidental or unauthorised loss of access10 to, or
           destruction of, personal data.

It should also be noted that, depending on the circumstances, a breach can concern confidentiality,
integrity and availability of personal data at the same time, as well as any combination of these.

Whilst determining if there has been a confidentiality or integrity breach is relatively clear cut, whether
there has been an availability breach may be less obvious. A breach will always be regarded as an
availability breach when there has been a permanent loss of, or destruction of, personal data.

Example

Examples of a loss of availability include where data has been deleted accidentally or by an unauthorised
person, or, in the example of securely encrypted data, the decryption key has been lost. In the event that
the controller cannot restore access to the data, for example, from a backup, then this is regarded as a
permanent loss of availability.

Loss of availability may also occur where there has been significant disruption to the normal service of an
organisation, for example, experiencing a power failure or denial of service attack, rendering personal
data unavailable.


          3. The possible consequences of a personal data breach
A breach can potentially have a range of significant adverse effects on individuals, which can result in
physical, material, or non-material damage. The GDPR explains that this can include loss of control over



8 See Recital 85.
9 See ISO/IEC 27000:2018, Information technology — Security techniques — Information security management
systems — Overview and vocabulary.
10
   Loss of access should be distinguished from unavailability in a planned situation such as scheduled system
maintenance for data security reasons.

Adopted                                                                                                        8
their personal data, limitation of their rights, discrimination, identity theft or fraud, financial loss,
unauthorised reversal of pseudonymisation, damage to reputation, and loss of confidentiality of personal
data protected by professional secrecy. It can also include any other significant economic or social
disadvantage to those individuals.11

Therefore, the GDPR requires the controller to assess the risk of harm12 that could result from a breach.
Depending on whether the controller considers that there is a risk (or a high risk) to individuals as a result
of a breach, it may have to notify the supervisory authority and communicate the breach to the data
subjects affected. The assessment of risk is required both for the purpose of notifying the competent SA
and for communicating the breach to affected individuals where appropriate. It is discussed in more detail
later in this document.

Example

Loss, or temporary unavailability of personal data, could be caused by a power outage that interrupts
normal processing, or by a ransomware attack that encrypts personal data. If the power outage or attack
is of sufficient duration to disrupt the normal processing of the data held, this may constitute an
availability breach. The breach is likely to be more significant if the data unavailability is for a longer rather
than a shorter period, for example if the data remains unavailable for several months rather than several
hours.

In the case of a ransomware attack, a confidentiality breach may also occur if the data is accessed for
malicious purposes by the attacker. For example, to gain a ransom, a hacker may access a hospital's
database, and take a copy of the patient database before the data is encrypted with a view to threatening
to publicise the data or to sell it if a ransom is not paid. This would be both a confidentiality breach and
an availability breach.

A temporary loss of availability of personal data (e.g. a power outage, or a ransomware attack, or denial
of service attack) is a type of security incident. However, it is necessary to consider whether the incident
results in personal data being compromised. If so, it is therefore a personal data breach and would need
to be addressed in accordance with Articles 33 and 34.

Accordingly, when considering a breach and its potential adverse effects on individuals, controllers should
also consider any wider consequences of a breach. This may happen for example when the personal data
breach is a symptom of a broader vulnerability of an IT system and indicates weaknesses in the processing
or in the data security in general.




11 See Recital 85.
12 Recitals 75 and 76 explain the concept of risk in the context of the GDPR. Recital 75 states that "The risk to the
rights and freedoms of natural persons, of varying likelihood and severity, may result from personal data processing
which could lead to physical, material or non-material damage, in particular: where the processing may give rise to
discrimination, identity theft or fraud, financial loss, damage to the reputation, loss of confidentiality of personal
data protected by professional secrecy, unauthorised reversal of pseudonymisation, or any other significant
economic or social disadvantage; …".

Adopted                                                                                                             9
     C.     Notification to the supervisory authority

          1. When to notify

              a)   Article 33 - Basic requirements
Article 33(1) states:

"In the case of a personal data breach, the controller shall without undue delay and, where feasible, not
later than 72 hours after having become aware of it, notify the personal data breach to the supervisory
authority competent in accordance with Article 55, unless the personal data breach is unlikely to result in
a risk to the rights and freedoms of natural persons. Where the notification to the supervisory authority is
not made within 72 hours, it shall be accompanied by reasons for the delay."

Recital 87 explains:

"It should be ascertained whether all appropriate technological protection and organisational measures
have been implemented to establish immediately whether a personal data breach has taken place and to
inform promptly the supervisory authority and the data subject."

Whilst the GDPR introduces the obligation to notify a breach, it is not a requirement that applies in all
circumstances:

      •   Notification is only required where the breach is likely to result in a risk to the rights and freedoms
          of individuals.

Recital 85 explains that:

"A personal data breach may, if not addressed in a timely and appropriate manner, result in physical,
material or non-material damage to natural persons such as loss of control over their personal data or
limitation of their rights, discrimination, identity theft or fraud, financial loss, unauthorised reversal of
pseudonymisation, damage to reputation, loss of confidentiality of personal data protected by
professional secrecy or any other significant economic or social disadvantage to the natural person
concerned."

This is made clear by Recital 87, which states:

"It should be ascertained whether all appropriate technological protection and organisational measures
have been implemented to establish immediately whether a personal data breach has taken place and to
inform promptly the supervisory authority and the data subject. The fact that the notification was made
without undue delay should be established taking into account in particular the nature and gravity of the
personal data breach and its consequences and adverse effects for the data subject. Such notification may
result in an intervention of the supervisory authority in accordance with its tasks and powers laid down in
this Regulation."

This means that after becoming aware of a breach, the controller should not only take steps to contain it,
but should also assess the risk (i.e. the likely impact on individuals, based on the severity of impact and
likelihood of occurrence) that could result. This assessment should help to establish what steps may need
to be taken: in terms of containment, whether to notify the competent SA, and whether to communicate
the breach to the affected individuals. This is likely to be part of an incident response plan that should be
in place.

Adopted                                                                                                    10
The requirement for the controller to document all breaches is covered in section I.E below.


              b)      When does a controller become "aware"?
As stated above, Article 33(1) requires notification, where feasible, within 72 hours "after having become
aware of it". This raises the question of when a controller can be considered to have become "aware" of
a breach.

The EDPB considers that a controller should be regarded as having become "aware" when that controller
has a reasonable degree of certainty that a security incident has occurred that has led to personal data
being compromised.

However, as indicated earlier, the GDPR requires the controller to implement all appropriate technical
protection and organisational measures to establish immediately whether a breach has taken place and
to inform promptly the SA and the data subjects. It also states that the notification was made "without
undue delay" and, "where feasible, not later than 72 hours".

The exact moment when a controller can be considered to be "aware" of a particular breach will depend
on the circumstances of the specific breach. In some cases, it will be relatively clear from the outset that
there has been a breach, whereas in others, it may take some time to establish if personal data have been
compromised. However, the emphasis should be on prompt action to investigate an incident to determine
whether personal data have indeed been breached, and if so, to take remedial action and make
notifications as required.

Examples

      1.     In the case of a loss of a USB key with unencrypted personal data it is often not possible to
             ascertain whether unauthorised persons gained access to that data. Nevertheless, even
             though the controller may not be able to establish if a confidentiality breach has taken place,
             such a case has to be notified as there is a reasonable degree of certainty that an availability
             breach has occurred; the controller would be "aware" when it realised the USB key had been
             lost.

      2.     A third party informs a controller that they have accidentally received the personal data of one
             of its customers and provides evidence of the unauthorised disclosure. As the controller has
             been presented with clear evidence of a confidentiality breach it can be in no doubt that it has
             become "aware".

      3.     A controller detects that there has been a possible intrusion into its network. The controller
             checks its systems to establish if personal data held on that system has been compromised
             and confirms this is the case. Once again, as the controller now has clear evidence of a breach
             there can be no doubt that it has become "aware".

      4.     A cybercriminal contacts the controller after having hacked its system in order to ask for a
             ransom. In this case, after checking its systems to confirm it has been attacked the controller
             has clear evidence that a breach has occurred and there is no doubt it is "aware".

After first being informed of a potential breach by an individual, a media organisation, or another source,
or when it has itself detected a security incident, the controller may undertake a short period of


Adopted                                                                                                  11
investigation in order to establish whether or not a breach has in fact occurred.13 During this period of
investigation the controller may not be regarded as being "aware". However, it is expected that the initial
investigation should begin as soon as possible and establish with a reasonable degree of certainty
whether a breach has taken place; a more detailed investigation can then follow.

Once the controller has become aware it shall, without undue delay and, where feasible, within 72 hours,
notify the supervisory authority where the breach is likely to pose a risk to individuals. If notification
cannot be achieved within 72 hours, reasons for the delay should accompany the notification.

When it is clear a breach has occurred but the full extent is not yet known, a notification can be made in
phases (see "Notification in phases" below in section I.C.2.b)).

Example

A storage media company that hosts data on behalf of several controllers detects a cyber attack on its
network. As a processor, it informs its controllers (see "Controller – Processor obligations" below in
section I.C.1.d)). After checking its own logs and having examined the attack characteristics, one of these
controllers establishes that personal data it holds on the servers of the processing company was accessed
during the attack. The controller takes appropriate action and notifies its SA.

Although the controller may undertake a short investigation, the onus is on it to ensure that any
investigation identifies with a reasonable degree of certainty whether there has been a breach and
whether it is likely to pose a risk to individuals. It is not intended to allow for lengthy investigations to be
undertaken before any notification is made.

Conversely, in some situations the controller will not be informed of a breach or will not become aware
of it for some time. If, after a period of time, it becomes apparent that the controller could have identified
the breach had it taken appropriate and reasonable measures, the controller cannot argue that it was not
aware in order to avoid any possible corrective powers or sanctions by the SA. In short, a key element of
any security policy should be to have measures in place that can detect and promptly contain a breach
and then inform relevant individuals/departments in the controller organisation that can then address
matters both in terms of remediation as well as addressing any notification requirements.14

Also, when engaging a processor, the contract between controller and processor should require the
processor to assist the controller in meeting all its obligations under the GDPR including the notification
of a breach to the SA.15


13 See ISO/IEC 27000:2018, Information technology — Security techniques — Information security management
systems — Overview and vocabulary. According to the ISO, an information security event is an identified occurrence
of a system, service or network state indicating a possible breach of information security policy or failure of controls,
or a previously unknown situation that may be security relevant. An information security incident is a single or a
series of unwanted or unexpected information security events that have a significant probability of compromising
business operations and threatening information security.
14 See ISO/IEC 27001:2017, Information technology — Security techniques — Information security management
systems — Requirements. This standard specifies the requirements for establishing, implementing, maintaining and
continually improving an information security management system within the context of the organisation.
15
   See Article 28 in particular and Article 33(2). See also EDPB Guidelines on controller-processor: EDPB Guidelines
07/2020 on the concepts of controller and processor in the GDPR, adopted on 2 September 2020, endorsed by the
EDPB on 5 March 2021. See also the examples of Article 28 clauses in the SCCs for processors (see Article 9 - Use

Adopted                                                                                                              12
Example

A person informs a controller that they have received an email impersonating the controller (spam email)
that contains their personal data, suggesting that the security of the controller has been compromised
somehow. The controller conducts a short preliminary investigation and discovers evidence of an
intrusion into its network, evidence that the confidentiality of personal data has been compromised, and
evidence of a phishing attack (evidence may include log files proving exfiltration of personal data, and
that personal data in the email could only have come from the controller's system(s)). In this case, the
controller would be clearly "aware" that a breach has occurred. The controller should have measures in
place to try to detect, and promptly investigate, a breach. In this example, the controller is alerted by a
third party and then investigates. In other cases, a controller might first become aware of a breach as a
result of its own monitoring, or following an audit or security review, or through information received
from an employee.

Regardless of when the controller becomes "aware", an important part of any security policy should be
to address any potential breach by detecting it, containing it, analysing the circumstances of it, and
minimising the consequences.

The controller should take into account the general principles of containment in order to manage the risks
of a breach. This may require immediate containment, for example if the breach is currently ongoing and
the priority is to stop it. This might also require a formal investigation. The controller is expected to
operate with reasonable diligence in the circumstances to contain the breach.

In cases where the investigation takes longer than 72 hours, notification should be made where feasible
within the 72 hours together with a reasoned justification for the delay. It is also possible to update the
notification within 72 hours without requiring justification.

As mentioned above, regardless of whether there is a risk to individuals, it is important for the controller
to document all breaches and take remedial actions. This is to enable the supervisory authority to exercise
its power in accordance with Article 58 to verify that the controller is compliant with Article 33.

The documentation of breaches should help the controller in several ways. First, it helps the controller to
meet its general obligations under Articles 5(2) (see section I.E) and 32 by identifying any current, or
potential future, vulnerabilities with its security. Second, it may help the controller to assist any regulatory
investigation, for example where the SA considers the possibility that the controller has not notified or
communicated a breach when it should have done, or if the SA considers any remedial measures the
controller has taken are insufficient. Third, it helps to foster organisational resilience and make
responding to any similar future breaches more efficient.

If after investigation a breach is found not to have occurred, the controller does not need to notify the SA
but should document this for its records.




of sub-processors in Module Two – Controller to Processor and in Module Three – Processor to Processor of the
Commission Implementing Decision (EU) 2021/914).

Adopted                                                                                                     13
              c)   Joint Controllers
Article 26 indicates that where two or more controllers jointly determine the purposes and means of
processing, they are joint controllers. Article 26 requires joint controllers to determine, in a transparent
manner, their respective responsibilities for ensuring compliance with the GDPR.

Therefore, the arrangement between joint controllers should indicate which controller will take the lead
on, or be responsible for, compliance with the breach notification obligations under the GDPR.

In the absence of such an arrangement, the joint controllers remain jointly responsible for compliance
with the GDPR, and therefore notification to the competent SA should be made by each of the joint
controllers if one of them becomes aware of a breach.

The EDPB considers that the arrangement between the joint controllers should clarify who among them
will have operational responsibility for notifying any breach. Each joint controller should be informed
promptly about any breaches that take place in the context of the respective processing.

Both (or all) of the joint controllers should consider the risk to the rights and freedoms of data subjects.
It may be the case that the risk only affects the processing of one of the controllers, or the actions of one
of the joint controllers could be relevant to mitigate or remove the risk for the affected individuals.

From a practical perspective, the joint controllers may have a contractual arrangement which provides
that a designated joint controller is responsible for making a breach notification to the competent SA on
behalf of the joint controllers. The fact that one of the joint controllers has been given operational
responsibility to notify or communicate a breach does not absolve the other joint controllers from their
responsibilities.


              d)   Controller – Processor obligations
A controller using a processor should ensure that the contract or other legal act between the controller
and the processor contains, among other things, specific provisions requiring the processor to assist the
controller to meet its obligations under the GDPR, including the obligation to notify breaches to the SA
and to affected individuals without undue delay (see Article 28(3)(f)).16

Article 33(2) makes clear that if a processor is being used by a controller and the processor becomes
aware of a breach of the personal data it is processing on behalf of the controller, it must notify the
controller "without undue delay". It should be noted that the processor does not need to first assess the
likelihood of risk before notifying the controller. This is the controller's responsibility. The processor need
only establish whether a breach has occurred and then notify the controller. The controller uses the
processor to achieve its purposes; therefore, in principle, the controller should be considered as "aware"
once the processor has informed it of the breach. The obligation on the processor to notify its controller
allows the controller to address the breach and to determine whether or not it is required to notify the
SA in accordance with Article 33(1) and the affected individuals in accordance with Article 34(1). The
controller may also want to investigate the breach along with its processor as the controller may not have
all the relevant facts relating to the incident, for example, where the processor is hosting off-site data
being processed by the controller.



16See also the examples of Article 28 clauses in the Standard Contractual Clauses (SCCs) for processors (see Article
9 - Use of sub-processors in Module Two – Controller to Processor and in Module Three – Processor to Processor
of the Commission Implementing Decision (EU) 2021/914).

Adopted                                                                                                         14
The GDPR does not provide an explicit time limit within which the processor must alert the controller,
other than that it must do so "without undue delay". Therefore, the EDPB recommends that the processor
promptly notifies the controller, with further information about the breach provided in phases as
information becomes available. This is important in order to help the controller to meet the requirement
of notification to the SA within 72 hours.

In line with Article 28(3)(f), the contract between the controller and processor should specify how the
requirements in Article 33(2) should be met in addition to other provisions under the GDPR. An
agreement may further develop this by setting out specific incident management procedures, specifying
initial and updated information about data breaches that must be provided by the processor to the
controller, and how the controller will be alerted (e.g., designated point of contact), the specific contacts
at the processor, when and how the controller will be notified/updated, including any events that
themselves must trigger a notification to the controller. The contract should also specify tasks to be
undertaken by either the processor or the controller such as triaging incidents, or determining whether
personal data was compromised in the first instance.17

Example

A controller uses a processor to handle customer mail outs. The processor discovers a cyber attack on its
database which stores controller customer contact details. When the processor becomes aware, it has a
duty to notify the controller. Depending on the facts of the attack the processor may not be in a position
to determine whether the controller customer details have been compromised. Therefore, it is obliged to
notify the controller that a cyber attack has taken place. The controller should work with the processor to
determine whether the controller customer details have been compromised.

In certain cases, the controller may not immediately know that the processor has alerted it to a breach.
For example, the controller may have been notified by the processor, but perhaps the person dealing with
such notifications is on leave, the alert was sent to the wrong email account, or the person receiving the
alert has failed to act on it.

Therefore, the contract between the controller and the processor should require that the processor
provides all relevant information to the controller so that the latter can react to a personal data breach.
To further strengthen this requirement, the contract may, in addition to a requirement for an initial
notification, require the processor to provide additional updates. The controller should ensure a
designated point of contact is available for the processor to use for notifications at all times and the
processor should ensure they record (and regularly check) the corresponding details.

If the processor in turn makes use of another processor (or sub-processor), and that sub-processor
becomes aware of a breach, it must promptly notify the processor and that processor must likewise
promptly notify the controller.

It should be noted that although the controller retains the duty to notify the SA, the controller may
appoint the processor to make the notification on the controller's behalf. This may be part of the service
the processor provides to the controller.




17
  See also the examples of Article 28 clauses in the Standard Contractual Clauses (SCCs) for processors (see Article
9 - Use of sub-processors in Module Two – Controller to Processor and in Module Three – Processor to Processor
of the Commission Implementing Decision (EU) 2021/914).

Adopted                                                                                                          15
             e)    Which data breaches must be notified to the supervisory authority?
Article 33(1) makes clear that if a breach occurs the controller has to consider the likelihood of a risk to
"the rights and freedoms of individuals" in order to determine whether it is required to notify the breach
to the SA. Such an assessment must be made on a case-by-case basis.

While Recital 85 makes clear that one of the purposes of the notification obligation is to limit damage to
individuals, being required to notify would also be expected to encourage the controller to ensure
appropriate security is in place to protect personal data.

The GDPR is not prescriptive as to what types of breaches may pose a risk to individuals. Instead, it is for
the controller to assess the risk, taking into account the nature, sensitivity and volume of personal data
and other factors relating to the breach.

Example
      1.    A risk may be considered as unlikely where the breach is unlikely to have an impact on
            individuals, or where the impact is unlikely to have a material effect on an individual's rights.
      For example, a controller may experience a brief power outage lasting several minutes on its
      customer helpline, meaning customers are unable to call the controller and access their records.
      This is a breach (as it is an availability incident) and is recordable, however it is unlikely to notify
      the SA, as the impact on data subjects does not reach the level of a risk to individuals.
      2.    On the other hand, it will be obvious that a notification will have to be made where, for example,
            a controller suffers a ransomware attack resulting in all its data being encrypted, no backups
            are available and the data has to be reconstituted from paper records. This will take several
            days and causes delays in customer deliveries and reduces employees' ability to do their jobs.
            This is a reportable breach as a significant impact on customers and employees can be shown.
       For further examples, see Annex B.

The mere existence of confidentiality, integrity or availability breaches does not necessarily mean that
notification is required. However, the controller should carry out a reasoned assessment taking into
account the type of breach, the nature, sensitivity and the amount of personal data compromised, and
how easy it is to identify the individuals affected. It should also assess the likelihood that the breach could
have an impact on individuals, the severity of such an impact, whether special characteristics of the
individuals are known that could affect the level of impact on them (for example, if children are the data
subjects), and what specific characteristics of the controller might also affect the level of impact, for
example, an organisation providing medical treatment.

This assessment should take into account whether the breach could lead to: identity theft or fraud,
financial loss, unauthorised reversal of pseudonymisation, damage to reputation, loss of confidentiality
of personal data protected by professional secrecy, or any other significant economic or social
disadvantage.

In line with the accountability principle, it is essential that the controller be able to demonstrate it has
correctly assessed whether a breach should be notified or not, and can justify not notifying the SA if it
considers a breach is unlikely to pose a risk to individuals.

Consequently, the EDPB recommends that the controller should document its reasoning when deciding
whether or not to notify, as it may be asked to produce this to the SA.



Adopted                                                                                                    16
With regard to the requirement to notify, the GDPR states in Article 33(1):

"In the case of a personal data breach, the controller shall without undue delay and, where feasible, not
later than 72 hours after having become aware of it, notify the personal data breach to the supervisory
authority competent in accordance with Article 55, unless the personal data breach is unlikely to result in
a risk to the rights and freedoms of natural persons."

This means that controllers are under an obligation to notify the SA of personal data breaches where such
breaches pose a risk to the rights and freedoms of individuals. Recital 87 explains further that the
controller should ascertain whether appropriate technical protection and organisational measures have
been implemented to establish immediately whether a personal data breach has taken place and to
inform promptly the SA and the data subject (the fact that notification was made without undue delay
should be established taking into account in particular the nature and gravity of the personal data breach
and its consequences and adverse effects for the data subject). Controllers should assess the risk at the
time they become aware of the breach.

Summary

The GDPR introduces the requirement for certain breaches to be notified to the competent supervisory
authority. For those breaches that need to be notified, the notification must be made "without undue
delay and, where feasible, not later than 72 hours after having become aware of it".

These two notification thresholds can be summarised as follows:

A. Where breach is unlikely to result in a risk to the rights and freedoms of natural persons
   - No notification is required
   - The controller must document the breach, which may be reviewed by the SA

Recitals 85 and 87 make clear that a breach requiring notification should be reported without undue delay
and within 72 hours. However, Recitals 85 and 87 also make clear that notification is not required if the
breach is unlikely to result in a risk to the rights and freedoms of individuals.

B. Where breach is likely to result in a risk to the rights and freedoms of natural persons
   - Notification to SA is required
   - Notification must be made without undue delay and, where feasible, not later than 72 hours
   - The controller must also document the breach

Article 33(1) makes clear that a breach likely to result in a risk to the rights and freedoms of individuals
should be notified. Recitals 85 and 87 explain that where notification is required, it must be made without
undue delay and, where feasible, not later than 72 hours after having become aware.

A breach notification to the SA need only include the following basic information: the categories and
approximate number of individuals and records concerned; the name and contact details of the data
protection officer (if applicable) or other contact point; the likely consequences of the breach; and the
measures taken or proposed to address the breach.

The information does not have to be provided all at once. Where the information cannot be provided at
the same time, the information may be provided in phases without undue further delay.




Adopted                                                                                                  17
When deciding whether to notify it may be helpful for the controller to refer to the European Union
Agency for Cybersecurity ("ENISA") recommendations on a technical methodology for the calculation of
breach severity for the purpose of fulfilling the notification obligations under the GDPR18 and the breach
notification requirement for providers of electronic communications under the ePrivacy Directive.19
However, the controller should be mindful that the obligation to notify is based on whether the breach is
likely to result in a risk to the rights and freedoms of individuals. The controller should note that the
thresholds for notifying a SA (risk) and for communicating the breach to an individual (high risk) are
different (see "Communicating to the individual" below).

Regardless of whether or not a breach has been notified, the controller must also consider whether
communication to the affected individuals is required (see section I.D below).

Notification of a breach to a SA is made in accordance with Article 55. This is addressed in section I.C.4
below.


Example

      1.    If a controller becomes aware of a breach at noon on a Tuesday, it should notify no later than
            noon on Friday (72 hours later). If the controller needs further time to collect additional
            information about the breach, it should make an initial notification with a reasoned justification
            for the delay and make subsequent updates as the investigation progresses.

      2.    The controller becomes aware of a breach at 9am on a Friday. The controller should, where
            feasible, complete the notification no later than 9am on Monday. If the controller has not
            completed the notification before 9am on Monday, it should notify its SA with a reasoned
            justification for the delay and continue to gather the necessary information, then make
            subsequent updates as the investigation progresses.

      3.    The controller becomes aware of a data breach on Friday. The data protection officer considers
            that the breach needs to be notified to the SA. The SA has an online reporting form. For reasons
            associated with the controller (for example, no appropriate representative available to
            complete the form), the breach is not notified until after the 72-hour deadline. In this case, the
            notification should be made as soon as possible, with a reasoned justification for the delay.




18 ENISA, Recommendations for a methodology of the assessment of severity of personal data breaches
(https://www.enisa.europa.eu/publications/dbn-severity).
19 ENISA, Recommendations on technical implementation guidelines of Article 4 – Security for personal data
breaches, 2012 (https://www.enisa.europa.eu/publications/art4_tech).

Adopted                                                                                                    18
             f)   Summary
The flowchart below summarises the notification requirements:




Adopted                                                            19
          2. Providing information to the supervisory authority

              a)   Information to be provided
Article 33(3) sets out the minimum content of a notification to the SA.

Pursuant to Article 33(3), the controller must:

"a) describe the nature of the personal data breach including where possible, the categories and
approximate number of data subjects concerned and the categories and approximate number of personal
data records concerned;

b) communicate the name and contact details of the data protection officer or other contact point where
more information can be obtained;

c) describe the likely consequences of the personal data breach;

d) describe the measures taken or proposed to be taken by the controller to address the personal data
breach, including, where appropriate, measures to mitigate its possible adverse effects."

Recital 85 explains:

"A personal data breach may, if not addressed in a timely and appropriate manner, result in physical,
material or non-material damage to natural persons such as loss of control over their personal data or
limitation of their rights, discrimination, identity theft or fraud, financial loss, unauthorised reversal of
pseudonymisation, damage to reputation, loss of confidentiality of personal data protected by
professional secrecy or any other significant economic or social disadvantage to the natural person
concerned. Therefore, as soon as the controller becomes aware that a personal data breach has occurred,
the controller should notify the personal data breach to the supervisory authority without undue delay
and, where feasible, not later than 72 hours after having become aware of it, unless the controller is able
to demonstrate, in accordance with the accountability principle, that the personal data breach is unlikely
to result in a risk to the rights and freedoms of natural persons. Where such notification cannot be
achieved within 72 hours, the reasons for the delay should accompany the notification and information
may be provided in phases without undue further delay."

The controller should explain the nature of the breach and outline measures recommended for the
individual to mitigate potential adverse effects. A notification is likely to include more detail on requests
regarding certain types of breaches which can result in significant consequences, such as ransomware
attacks or exfiltration of sensitive data. This may include, for example, details of: the malware or
ransomware used; evidence of any data exfiltration; time and nature of any compromise; attack vectors
exploited (e.g. phishing attack); the types of data and records compromised (e.g. health records); the
types of vulnerabilities that the controller was exposed to and that were exploited; the measures the
controller has taken to address the vulnerabilities (this might include both technical measures in relation
to the vulnerability exploited, and measures to prevent any similar attacks in the future, such as additional
staff training on phishing attacks).

As part of the notification, the controller may describe the categories of personal data affected by the
breach (for example, financial data, health records, student exam results, contact details) and the types
of personal data records affected (for example, log files, audit records, system records, incident reports).
It also may be relevant to state, whether the personal data breached included any special categories of

Adopted                                                                                                   20
personal data.20 Relevant information may also include details of any security measures such as
encryption of data and any use of pseudonymisation.

When describing the likely consequences of the breach, the controller should explain the harm that
specific categories of data subjects might face, such as potential identity fraud, physical harm, or
psychological distress. This is to help the SA better understand the nature and the severity of the breach.

Based on the analysis, the notification should describe the measures taken to remediate the breach, as
well as any mitigation measures taken to reduce the harm to affected data subjects.

Some SAs may utilise online reporting portals or specific forms that must be used to make a notification.
Such mechanisms can assist controllers to more easily provide all the relevant details for their specific
breach. This may include online forms that prompt the controller to enter specific information about a
breach. Such forms may also provide a structured template for the controller to make further updates or
add to the information already provided to the SA. Controllers should check with the relevant SA for
details of how to notify.

When reporting a breach, the controller may, where appropriate, ask the SA for advice on whether the
data subjects concerned need to be informed. For example, this may be useful when the controller is
unsure if the personal data has been accessed or disclosed. Regardless of whether the SA advises whether
the data subjects should be informed, the responsibility for the communication decision remains with the
controller.

Where a controller seeks advice from the SA on whether to communicate a breach to the individuals
affected, or on what message should be sent to them, the SA has no duty to give such advice. In any event,
the controller remains responsible for taking appropriate actions in response to any breach.


              b)   Notification in phases
Depending on the nature of the breach, further investigation by the controller may be necessary to
establish all the relevant facts relating to the incident. Article 33(4) therefore states:

"Where, and in so far as, it is not possible to provide the information at the same time, the information
may be provided in phases without undue further delay."

This means a controller may not have all of the necessary information concerning a breach within 72
hours of becoming aware of it – as full details of the impact of a breach may not always be available during
this initial period. Hence, the GDPR allows for notifications to be made in phases, including where
applicable after 72 hours. This may happen for example where the controller is still investigating the
breach and does not have all the relevant facts available.

Recital 85 also makes clear that:

"[w]here such notification cannot be achieved within 72 hours, the reasons for the delay should
accompany the notification and information may be provided in phases without undue further delay."

It is acceptable for a controller to make an initial notification to the SA within 72 hours and then add
further details over time. If a controller makes an initial notification within 72 hours, but subsequently


20
     See Article 9 GDPR.

Adopted                                                                                                  21
seeks to add to the information it initially submitted, a reasoned justification is not required (unless the
update is provided more than 72 hours after the initial notification).

This also means the controller should be proactive and keep in touch with the SA. Updates may be
required when the investigation reveals new details about the breach or changes the controller's view of
its impact.

If the scope or severity of a breach changes, notifications should be updated to include any material facts.
For example, if a controller subsequently discovers that a breach is more significant than first thought or
certain mitigation measures are not as effective as expected, this may affect the communication to
individuals. Accordingly, such a reassessment could change the original notification to the SA or the
original decision as to whether individuals should be informed.

Where a breach involves joint controllers, regardless of any arrangement between them, each controller
should ensure the relevant details are available to make a full notification to the SA (see "Joint Controllers"
above in section I.C.1.c)).


          3. Delayed notifications
Article 33(1) is clear that if notification cannot be achieved within 72 hours, reasons for the delay should
be given. The same applies to notification in phases (see Article 33(4) and section I.C.2.b)).

Along with phased notification, this provision acknowledges that a controller may not always be able to
notify a breach within that time period and that a delayed notification may be permissible.

Example

A controller may have determined that a breach affecting only a few individuals is likely to result in a risk
and requires notification. However, a delayed notification may be acceptable if the controller is suffering
from a series of similar small-scale breaches affecting different individuals over a short period of time.
This might be the case where the controller identifies a weakness in a common security feature that allows
an unauthorised person to access personal data, the weakness being exploited as the controller identifies
it, or where different unauthorised persons are repeatedly accessing a small amount of personal data. In
such cases, the controller should be endeavouring to correct the flaw, but whilst it does so the original
breach may "repeat" several more times with different data subjects being affected. Rather than notifying
each breach as a separate breach (which would lead to a series of separate notifications made closely
together) the controller may instead "bundle" together the notifications for these similar breaches over
a short time period, meaning that the 72-hour deadline for notification may be exceeded. The cause,
including the nature of breaches and the number of data subjects affected, should be analysed, in order
to develop a sustainable solution to the breach. This must be carefully supervised and it is recommended
to be done in conjunction with the SA.

In any event a controller that fails to notify within 72 hours should explain the reasons for delay.

In some cases, for example when a notified breach is subsequently discovered to be less severe than
initially thought, the controller is expected to update the SA. The controller should maintain records of all
breaches, including those it did not notify for articulated reasons.

Recital 88 explains:



Adopted                                                                                                    22
"In setting detailed rules concerning the format and procedures applicable to the notification of personal
data breaches, due consideration should be given to the circumstances of that breach, including whether
or not personal data had been protected by appropriate technical protection measures, effectively
limiting the likelihood of identity fraud or other forms of misuse. Moreover, such rules and procedures
should take into account the legitimate interests of law-enforcement authorities where early disclosure
could unnecessarily hamper the investigation of the circumstances of a personal data breach."

Therefore, when providing information to the SA, the controller should explain if there are any legitimate
reasons why notification has been delayed, for example why an incomplete notification has been made.
The SA may then take into account any relevant reasons for the delay when it considers whether to take
action under Article 83(4)(a) in relation to a failure to notify or an incomplete notification.


          4. Cross-border breaches and breaches at non-EU establishments

              a)   To which supervisory authority (or authorities) should the notification be
                   made if there is a cross-border personal data breach?
Article 33(1) requires notification to be made to the SA "competent in accordance with Article 55".
Chapter VII of the GDPR establishes the one-stop-shop mechanism which means that, where a breach is
cross-border, the controller only has to notify its lead SA. Where a controller does not have a main
establishment in the EEA, it must notify all the SAs.

Article 4(23) provides:

" 'cross-border processing' means either:
(a) processing of personal data which takes place in the context of the activities of establishments in more
than one Member State of a controller or processor in the Union where the controller or processor is
established in more than one Member State; or
(b) processing of personal data which takes place in the context of the activities of a single establishment
of a controller or processor in the Union but which substantially affects or is likely to substantially affect
data subjects in more than one Member State."

In the case of a cross-border processing only, a breach affecting data subjects in more than one Member
State should be notified to the competent lead SA. If the controller is unsure of who the lead SA is, at a
minimum it should notify the local SA where the breach has occurred.21

Controllers should also consider any other local or sector-specific legal obligations that may be in place in
addition to Article 33, which may require them to notify other bodies/authorities, including the local SA
of a breach.

Recital 22 explains:

"Processing of personal data in the context of the activities of an establishment of a controller or a
processor in the Union should be carried out in accordance with this Regulation, regardless of whether
the processing itself takes place within the Union. Establishment implies the effective and real exercise of
activity through stable arrangements. The legal form of such arrangements, whether through a branch
or a subsidiary with a legal personality, is not the determining factor in that respect."


21   See Article 4(16) and Recital 36 GDPR.

Adopted                                                                                                    23
It should be noted that the main establishment of a controller in the EU may not be the establishment
which is responsible for the processing of the affected personal data. For example, a breach may be
related to processing of personal data carried out by a branch of the controller in a country other than
the country where the controller has its main establishment. Controllers should be aware of this and
ensure that all relevant information is available to the person(s) or team(s) charged with breach
notification.

With reference to EDPB Guidelines 2/2022 on the application of Article 60 GDPR,22 the controller should
identify which SA is its lead SA (if any) and to notify any breach to that SA.23 This means making a
notification to the SA of the Member State where the controller's main establishment is located.24 If the
controller is unsure of who the lead SA is, the controller should notify the local SA in the Member State
where the breach has occurred.

A data processor that has suffered a data breach should notify the controller, not the controller's lead SA.
See "Controller – Processor obligations" above (section I.C.1.d)).

Where the controller has already identified its lead SA (because it knows that it performs cross-border
processing) and a cross-border breach occurs, it should notify its lead SA.25

Example

      1.    A controller has establishments in Member States B, C, and F but its main establishment is in
            Member State C. If a breach that affects only data subjects in Member States B and F occurs in
            Member State B, this will still be a cross-border processing. Accordingly, the breach must be
            reported to the SA in Member State C as this is the lead SA. However, the controller may have
            other local or sector-specific legal obligations that require it to notify the breach to the local SA
            in Member State B or F.

      2.    A controller is established in only one Member State, and its processing only affects data
            subjects in that Member State. A breach occurs affecting only those data subjects. As this is not
            a cross-border processing, the controller should notify the SA in the Member State where it is
            established. The one-stop-shop mechanism is not applicable in this case.

      3.    A controller is established in only one Member State, and its processing affects data subjects in
            several Member States. A breach occurs affecting data subjects in all those Member States. As
            this is a cross-border processing, the controller has one lead supervisory authority, which is the
            SA in the Member State where it has its only establishment. Accordingly, the controller should
            notify the breach to that lead SA. The one-stop-shop mechanism is applicable to any further
            investigation of the breach.




22
   https://edpb.europa.eu/our-work-tools/our-documents/guidelines/guidelines-022022-application-article-60-
gdpr_en
23
   For guidance on determining the lead SA, see the EDPB Guidelines on the lead supervisory authority in respect
of cross-border processing: https://edpb.europa.eu/our-work-tools/our-documents/guidelines/guidelines-identify-
controller-or-processors-lead_en
24
   See Article 56(1).
25
   See EDPB Guidelines on the lead supervisory authority in respect of cross-border processing (WP 244 rev.01).

Adopted                                                                                                      24
Clearly, in some cases, a breach may concern solely data subjects in the Member State where a controller
has its only establishment and may have no cross-border element, and only the SA in that Member State
would be competent.26

Whilst not a GDPR requirement, the controller may consider giving advance warning to the SA(s) of the
Member States of the data subjects affected by a cross-border processing breach, in advance of the
notification of the breach to the lead SA. This is on the understanding that the lead SA, as a matter of
good administrative practice, will also inform the SA(s) of the affected Member States of the breach
notification.

In cases of non-cross-border processing, for example where the controller has an establishment in only
one Member State and where processing affects only data subjects in that Member State, a breach should
be notified to the SA of the Member State where it is established, and there is no requirement for the
controller to notify another SA where affected data subjects reside.

Regardless of whether the processing is cross-border or not, Article 33 does not prevent the controller
from also notifying the SA of any Member State affected by the breach. This may be appropriate in certain
circumstances, for example where the controller has a legal obligation under local law to notify a specific
body (for example, a SA, or a sectoral body, or the competent authority for electronic communications).
It may be a condition for processing personal data that the controller must register with the national SA
or get authorisation for the processing activities. In such cases, Article 33 does not prevent the controller
from also notifying any other SAs or authorities or bodies affected by or with an interest in the breach in
accordance with local law.

Further, where there is no lead SA, the breach must be notified to every SA in the Member States where
data subjects have been affected.27

Example

An insurance company has no establishment within the EU but only in a third country. The controller
offers services to EU data subjects, and processes their personal data. It has appointed an Article 27
representative in Belgium. It suffers a breach that affects data subjects in Belgium, France, Germany and
Italy. As the insurance company is not established in the EEA, the one-stop-shop mechanism will not
apply, and therefore, it has no lead SA.28 Accordingly, the insurance company must notify the SAs of each
of the Member States where the affected data subjects reside. In this case, the breach must be notified to
Belgium, France, Germany and Italy.

Depending on the location of the data subjects affected by a breach, the controller may have to notify
more than one SA. A controller should have prepared for such a situation as part of its breach response
plan. To facilitate notification, it may wish to use a standardised notification template to help with the
practicalities of notifying more than one SA.




26
   See Articles 55 and 56 GDPR.
27
   The EDPB may wish to consider how this obligation could be facilitated and streamlined.
28
   See Article 56(1) GDPR.

Adopted                                                                                                 25
              b)   Breaches at non-EU establishments
Recital 22 makes clear that whether personal data is processed within the EU or not, processing activities
carried out by a controller established in the EU should be carried out in accordance with the GDPR.

In circumstances where a controller subject to the GDPR is also established outside the EU, and the
breach relates to activities being carried out at the third country establishment, the controller must still
comply with its notification requirements under Article 33. The fact that the breach occurred outside the
EU does not absolve the controller from notification requirements.

Example

A controller based in the EEA operates a help desk facility in a third country (a processor). The help desk
facility responds to queries from customers. A cyber attack on the help desk facility results in personal
data being transferred to the attacker. The controller has suffered a personal data breach, and must assess
whether to notify. If notification is required it will be to the SA of the Member State of the EU
establishment.

For controllers whose activities fall within the territorial scope of the GDPR (Article 3) but are not
established in the EU, the one-stop-shop mechanism does not apply, even where an Article 27
representative has been appointed, and a lead SA does not exist.29 A breach occurring at such
establishments must be notified to the SAs of all Member States where affected data subjects reside.

The GDPR does not require a breach notification to be made by a particular person or department within
the controller, nor does it prescribe the form that such notifications should take. It is recommended that
if a representative is designated, a breach notification to a SA should identify the Article 27 representative
as a point of contact for the controller.30

Recital 80 explains that "in order to ensure compliance with the requirements of this Regulation in respect
of the processing to be carried out by the processor on behalf of the controller, when entrusting a
processor with processing activities, the controller should use only processors providing sufficient
guarantees, in particular in terms of expert knowledge, reliability and resources, to implement technical
and organisational measures which will meet the requirements of this Regulation, including for the
security of processing."

If a breach occurs in the EU, but the controller is not established in the EU and has no representative in
the EU (it should have designated a representative), it must still notify the competent SA, i.e. the SA of
the Member State where data subjects have been affected. If a breach occurs outside the EU, but relates
to the processing of personal data of data subjects within the EU and falls within the scope of the GDPR,
breach notification obligations will apply. Notification in this case should be made to the SA of the
Member State where affected data subjects reside.

The GDPR provides that "Where applicable, the controller shall provide the contact details of the data
protection officer or other contact point where more information can be obtained". If the controller does
not have a DPO, or another designated contact point, the Article 27 representative should be identifiable
as a person who can be contacted for queries relating to data protection generally and in relation to the
notified breach. If the controller considers it expedient to do so, it is always free to identify additional

29
   See Article 56(1) and recital 127 GDPR.
30
   Article 27(4).

Adopted                                                                                                    26
points of contact for the SA to communicate with, for example, if the breach occurred outside the EU at
another establishment and a representative of that establishment is best placed to provide answers to
any further queries, that representative may be named in addition to the Article 27 representative.

See the EDPB Guidelines on territorial scope to determine whether processing falls within Article 3.31


    D.      Communication to the data subject

          1. Informing individuals
When the breach is likely to result in a high risk to the rights and freedoms of the individual, the controller
shall also communicate the breach to the affected individuals.

Article 34(1) provides:

"When the personal data breach is likely to result in a high risk to the rights and freedoms of natural
persons, the controller shall communicate the personal data breach to the data subject without undue
delay."

As with notifying a SA, the controller should communicate a breach to the affected individuals without
undue delay, which means as soon as possible. The main objective of notification to the affected
individuals is to provide specific information about steps they can take to protect themselves.

As noted above, depending on the nature of the breach, and the risks posed, timely communication will
help individuals take steps to protect themselves from the any negative consequences of the breach.

However, the GDPR does not give a specific timeframe within which the individuals should be informed.
It only says the individuals should be informed "without undue delay". Therefore, it is up to the controller
to decide, with the advice of the DPO if applicable, on the timeframe for informing affected data subjects.
This is likely to be influenced by the facts of each case. Failure to adequately inform individuals in a timely
manner about a breach that affects them could result in a controller receiving a sanction under the GDPR
or a direction to communicate the breach. It may also be relevant if an individual complains to the SA
about the handling of a breach.

Recital 86 explains:

"Such communications to data subjects should be made as soon as reasonably feasible and in close
cooperation with the supervisory authority, respecting guidance provided by it or by other relevant
authorities such as law-enforcement authorities. For example, the need to mitigate an immediate risk of
damage would call for prompt communication with data subjects whereas the need to implement
appropriate measures against continuing or similar personal data breaches may justify more time for
communication."

In certain cases, it may also be relevant for a controller to contact individuals so that they are aware of
the possibility of harm – if there are appropriate steps they can take themselves to protect from harm
such as changing their password.



31
  EDPB, Guidelines 3/2018 on the territorial scope of the GDPR (Article 3) – Version adopted after public
consultation (v.2.1), adopted on 7 January 2020.

Adopted                                                                                                     27
Communication with affected individuals allows them to understand what has happened, and if
necessary, take steps to protect themselves. This could include changing login details or passwords, being
alert to phishing or other fraudulent messages, or being alert to misuse of their personal data.

As mentioned above, a controller may first want to assess the seriousness of the breach and consider
whether communication to the affected individuals is necessary. Clearly, a SA has a role to play here: the
SA may advise or require the controller to inform the individuals, depending on the circumstances of the
case.32 In any event, the decision on whether to communicate the breach to the data subject is the
controller's responsibility, irrespective of whether the SA has required the communication to be made.

Regardless of whether or not a notification is made to the SA, the controller should assess whether
affected individuals should also be informed.


          2. Information to be provided
Article 34(2) explains:

"The communication to the data subject referred to in paragraph 1 shall describe in clear and plain
language the nature of the personal data breach and contain at least the information and measures
referred to in points (b), (c) and (d) of Article 33(3)."

As a minimum, the communication should include the following (see Article 33(3)(b), (c) and (d)):

     •    a description of the nature of the breach;
     •    the name and contact details of the data protection officer or other contact point;
     •    a description of the likely consequences of the breach; and
     •    a description of the measures taken or proposed to be taken by the controller to address the
          breach, including, where appropriate, measures to mitigate its possible adverse effects.

Example

A controller's online banking system is compromised. The attacker has exploited a vulnerability that has
allowed the attacker to intercept the account login credentials of all customers that logged in during a
specified time window. Not all the bank's customers will have logged in during the affected period, but
the controller cannot identify which customers logged in during that period. Although this is a cross-border
processing, depending on the location of the bank, not all affected customers are likely to be in the same
Member State. In this case, the controller should include information in its communication to affected
individuals explaining that the data breach has been reported to the SA, that all individuals whose
credentials may have been accessed are being informed of the breach and the steps they should take to
mitigate any potential adverse effects.

The controller may also use the opportunity to provide practical information and advice to affected
individuals. This advice and information could include, for example:

     •    action the controller has taken;
     •    action individuals can take to reduce or minimise any adverse effects of the breach (for example,
          if passwords have been compromised they should change them);


32
     See Article 58(2)(e).

Adopted                                                                                                 28
    •   relevant points of contact for individuals affected (for example, a dedicated help number,
        specific contact hours for a helpline, or a dedicated website/microsite with relevant information);
    •   a link to the most current information about the breach.

Dedicated communications with individuals should be separate from normal communications with them,
such as newsletters or standard messages. Communication about a breach should be clear and
transparent. To ensure a consistent message to all data subjects, the controller should consider informing
the media to help inform those data subjects that may not be contacted directly (for example, in the case
where the controller does not have contact details for all affected data subjects).

Example

The bank controller in the above example should explain in its communication how the affected individuals
can protect themselves (e.g. by changing their password and considering enabling their bank's multi-factor
authentication feature), and what the bank has done to remedy the breach, and, if appropriate,
information about how they can claim appropriate compensation. In addition, this information should be
made available on the bank's website.


          3. How should individuals be contacted?
Communication of a breach to the affected individuals should be done in a way that maximises the chance
that information will be communicated to all those affected:

    •   Recital 86 refers to communications being done in accordance with any guidance from the SA or
        other relevant authority (e.g., law enforcement);

    •   Article 34(3)(c) says "if it would involve disproportionate effort" to contact individuals
        separately, there should be "a public communication or similar measure whereby the data
        subjects are informed in an equally effective manner." This may involve, for example, a message
        on the company's website or on social media, or other similar means.

A message should be dedicated to the breach and not combined with other information, such as updates,
newsletters, or standard messages. The message should be separate so that it is clear and transparent.33

The controller may use various channels of communication. For example, email, SMS, direct message,
prominent website banners or notifications, postal communications, and advertisements in print media.
The controller should take all reasonable measures to reach all affected data subjects. The controller
should, if necessary, combine channels if doing so will increase the chance of communicating effectively
with all affected individuals.

It should be noted that the GDPR mentions specific measures that controllers should take into account
when communicating information. The EDPB would also encourage the controller to set up its
communication so that if further information becomes available (for example, an update on the status of
a breach, or an update to the investigation) it can be easily included in that communication.




33
  See Guidelines on Transparency under Regulation 2016/679 (wp260 rev.01). Available here:
http://ec.europa.eu/newsroom/article29/item-detail.cfm?item_id=622227.

Adopted                                                                                                29
          4. Conditions when communication is not required
Article 34(3) provides three conditions where communication to the individual is not required:

"(a) the controller has implemented appropriate technical and organisational protection measures, and
those measures were applied to the personal data affected by the personal data breach, in particular
those that render the personal data unintelligible to any person who is not authorised to access it, such
as encryption;

(b) the controller has taken subsequent measures which ensure that the high risk to the rights and
freedoms of data subjects referred to in paragraph 1 is no longer likely to materialise;

(c) it would involve disproportionate effort. In such a case, there shall instead be a public communication
or similar measure whereby the data subjects are informed in an equally effective manner."

The first point, (a), recognises that the encrypted personal data would be unintelligible if accessed without
the decryption key. While encryption can render data unintelligible and therefore protect individuals
affected by a breach, the controller should carefully consider whether all appropriate and state of the art
measures were in place when the breach occurred. Using encryption appropriate to the risk may mean
that when a breach occurs it is less likely to present a risk to the rights and freedoms of the individuals
concerned and consequently notification may not be required. However, this should be assessed on a
case-by-case basis using a risk-based approach and the protection employed must be appropriate to the
risk.

Example

A controller uses a secure email service to provide routine communications to its customers that contain
banking details. The email service has end-to-end encryption enabled.

In this case, if a hacker breaches the email service, the data would still be encrypted.

End-to-end encryption reduces the risk of data being compromised in transit. However, encryption is not
foolproof. For example, if the data has been sent to, or held by, the intended recipient on the (unencrypted)
email service of the receiving party, the data may be at risk. Furthermore, if the encryption keys are
compromised, the encrypted data may then be at risk.

Regarding point (b), such measures could include, in some circumstances, immediate action taken by the
controller on becoming aware of the breach. Such actions may mean that the high risk no longer
materialises, and so it would not be necessary to communicate the breach to the affected individuals. For
example, if personal data has been accessed for the purposes of committing fraud, but if the controller is
able to identify this and prevent it before any damage is done, it may be that there is no high risk to the
affected individuals. However, if the controller had been alerted to the breach by a third party (e.g. law
enforcement) who were investigating with a view to apprehending the perpetrators, the controller would
still be required to notify the individuals, and failure to do so could result in a fine.

Example

A retail company suffers a cyber attack in which the contact details of some customers are exfiltrated. In
this case, the customers whose data has been stolen would need to be alerted to the threat to their privacy
posed by the release of their contact details. On discovering the attack, the controller is able to take
remedial action to block the attack and prevent any further data from being taken. The controller is also
able to determine how long the attack has been taking place and what specific data was accessed. In this

Adopted                                                                                                   30
case, even though the controller has taken action to prevent a further data breach (which should help to
reassure its customers), the individuals concerned will still need to be informed as personal data has
already been released.

Regarding point (c), whether the data subjects can be contacted directly may depend upon the controller
having accurate contact details for those affected; if there are reasonable alternative means of reaching
individuals then the controller should use these as appropriate. For example, an organisation that only
provides an online service and collects email addresses rather than phone numbers or physical addresses,
would normally contact the affected individuals by email. When deciding whether to take additional steps
to contact individuals, the controller should have regard to the nature of the breach, the risk to affected
data subjects, and the desire to mitigate the effects on them. As noted above, there may be circumstances
where a controller is not able to identify all the affected data subjects. In such cases, using the appropriate
method to make the communication public will ensure that those affected are made aware and can take
action accordingly.

When a public communication is made, this should use an appropriate means to contact the affected
individuals. Such means should take into account the nature of the personal data breach and its
consequences. For example, this might include a prominent message on a website or a press release
explaining the situation and referring readers to an explanation online, together with contact details.


    E.     Documenting breaches
Article 33(5) indicates that the controller has to document the facts relating to the breach, its effects, and
the remedial action taken. This forms part of its obligations under Article 5(2), which requires controllers
to be accountable for compliance with the principles of Article 5(1). It should be noted that the duty to
document (Article 33(5)) applies even in cases that do not meet the threshold for notification.

Article 33(5) provides:

"The controller shall document any personal data breaches, comprising the facts relating to the personal
data breach, its effects and the remedial action taken. That documentation shall enable the supervisory
authority to verify compliance with this Article."

The controller is required to maintain records of breaches regardless of whether or not they are notifiable.
This documentation requirement applies to all breaches, whether or not they pose a risk to the rights and
freedoms of individuals (see section I.C.1.e) above).

The requirement relates to Article 5(2) (Accountability). Recital 82 explains in this regard that:

"[The controller is required] to maintain records in order to prove compliance with this Regulation. The
controller should be required to maintain records of processing operations under its responsibility and the
processor should be required to maintain records of categories of processing activities under its
responsibility. The controller or the processor should be obliged to cooperate with the supervisory authority
and make these records available to it on request so that it might serve for monitoring those processing
operations."

As part of this documentation obligation, the controller may record the details of the breach, which can
include its causes, what took place, and the personal data affected. It should also include the effects and
consequences of the breach as well as the remedial action taken by the controller.


Adopted                                                                                                    31
It is recommended that the controller also documents its reasoning for the decisions taken in response to
a breach. In particular, where a breach is not notified, a justification for that decision should be
documented. This should include reasons why the controller considers the breach is unlikely to result in a
risk to the rights and freedoms of individuals. Alternatively, if the controller considers that any of the
conditions in Article 34(3) are met, and so communication to the individual is not required, this reasoning
should also be documented. If the controller holds the view that any of the conditions in Article 34(3) are
not met, but nonetheless decides not to communicate to the data subject, that reasoning should also be
documented.

The controller may include additional information in the breach record. For example, a record of breach
events may include other relevant information, for example if information is known about the
perpetrators or where an incident is potentially related to a previous incident.

When the controller notifies a breach to a SA it may be required to provide evidence such as a report
describing the breach and relevant log files.


    F.     Risk assessment and examples of various breach scenarios

          1. Risk assessment
In the context of Article 33, the GDPR refers to "a risk" and a controller must notify a breach to the SA
unless the breach is unlikely to result in a risk to the rights and freedoms of individuals.

In the context of Article 34, the GDPR refers to "a high risk" and the controller must communicate a breach
to the affected individuals where it is likely to result in a high risk to the rights and freedoms of individuals.

The controller should assess the risk of harm to individuals from a breach at the time it becomes aware
of a breach. Different scenarios involving breaches should be considered in advance as part of the
controller's overall incident response plan.

Section 2 below outlines the different types of impact that a breach may have and the factors to take into
account when assessing risk. Section 3 below briefly summarises different categories of data breach.
Annex B provides practical examples of likely impacts of breaches.

This risk assessment may also help the controller to take effective measures to contain and handle a
particular breach when it occurs and to decide who needs to be notified.


          2. Conditions for risk
Recitals 75 and 76 provide helpful guidance in terms of assessing risk under the GDPR.

Recital 75 explains:

"The risk to the rights and freedoms of natural persons, of varying likelihood and severity, may result from
personal data processing which could lead to physical, material or non-material damage, in particular:
where the processing may give rise to discrimination, identity theft or fraud, financial loss, damage to the
reputation, loss of confidentiality of personal data protected by professional secrecy, unauthorised
reversal of pseudonymisation, or any other significant economic or social disadvantage; where data
subjects might be deprived of their rights and freedoms or prevented from exercising control over their
personal data; where personal data are processed which reveal racial or ethnic origin, political opinions,
religion or philosophical beliefs, trade union membership, and the processing of genetic data, data
concerning health or data concerning sex life or criminal convictions and offences or related security

Adopted                                                                                                      32
measures; where personal aspects are evaluated, in particular analysing or predicting aspects concerning
performance at work, economic situation, health, personal preferences or interests, reliability or
behaviour, location or movements, in order to create or use personal profiles; where personal data of
vulnerable natural persons, in particular of children, are processed; or where processing involves a large
amount of personal data and affects a large number of data subjects."

Recital 76 explains:

"The likelihood and severity of the risk to the rights and freedoms of the data subject should be determined
by reference to the nature, scope, context and purposes of the processing. Risk should be evaluated on
the basis of an objective assessment, by which it is established whether data processing operations involve
a risk or a high risk."

These recitals make clear that the focus is on the risk to individuals and the potential impact on their
rights and freedoms. The GDPR explains that this can include physical, material or non-material damage
to individuals, particularly where the breach may result in discrimination, identity theft or fraud, financial
loss, damage to reputation, loss of confidentiality of personal data protected by professional secrecy,
unauthorised reversal of pseudonymisation, or any other significant economic or social disadvantage. It
can also include deprivation of rights and freedoms and preventing data subjects from exercising control
over their personal data.

Risk is a factor of the potential harm to individuals in a breach, together with the likelihood of occurrence
of that harm. When considering the potential impact the following factors should be considered:

     •    The type of breach that has occurred (e.g., a confidentiality breach may have more or different
          impacts than an availability breach);
     •    The nature, sensitivity and volume of personal data (see further detail below);
     •    Ease of identification of individuals from the compromised personal data (i.e., how easily could
          the data be matched against other data about the individual or be otherwise used to identify
          them directly?);
     •    Severity of consequences for individuals, such as threat to life, impact on financial situation,
          impact on reputation, impact of psychological harm;
     •    Special characteristics of the individual, such as vulnerability of individual, for example, children,
          elderly, or other groups (for example, factors relating to the profession of the individual that
          might make them more of a target for harm or harassment);
     •    Special characteristics of the data controller, for example, an organisation providing medical
          treatment;
     •    The number of affected individuals; and
     •    Some general points to consider: for example, the risk of identification may be negligible on its
          own, but may be significant when combined with other information; the risk may be higher when
          personal data is sent to a mass mailing list as opposed to to a single individual.

The nature of the personal data subject to a breach can also affect the level of risk to affected individuals.
For example, special categories of data, or data which is of an otherwise sensitive nature, can result in
more harm to an individual if misused.

After considering the factors above, the controller should then assess the likelihood of harm occurring.
The higher the potential severity of harm, the greater the significance of even a slight risk. If a breach has
not been notified because the controller decided it was unlikely to result in harm, the controller should


Adopted                                                                                                     33
be able to demonstrate to the SA how it reached this conclusion: see section I.E. above, regarding
documentation.

For further guidance on how to assess risk, see ENISA's recommendations on a methodology of the
assessment of severity.34


           3. Categories of data breaches
Breaches fall into a limited number of broad categories. Some scenarios are more common, and so
controllers should have guidelines for evaluating and responding to breaches that fall within those
categories. Annex B contains a number of examples of common breaches and an analysis of how they
should be assessed. Details are as follows:


               a)    Ransomware attacks
Ransomware is a type of malicious code ("malware") that encrypts personal data and then requires the
user to pay a ransom to obtain access to the decryption key. Ransomware can be delivered to a system
through various means, most often via malicious attachments to emails, but can also be deployed through
compromised website downloads or social media. In addition to encrypting data on a targeted system,
attackers are increasingly using ransomware to extract data from targeted systems prior to encryption
and before the victim is aware the attack has occurred. This can result in a combined confidentiality and
availability breach.

If a controller is the victim of a ransomware attack, this is a personal data breach (more specifically,
normally an availability breach). An investigation should try to ascertain the root cause of the breach, and
whether data has been exfiltrated.

Once the nature of the breach is confirmed, it must still be assessed to determine whether notification to
the SA and to individuals is required. Key factors in considering the risk associated with a ransomware
attack are: whether data has been exfiltrated or copied; whether there is a backup of the data that the
controller can restore; and what is the nature of the data involved. If there is clear evidence that the data
has not been copied or extracted from the system, and if the data is encrypted within the controller's
system, the breach is likely to be an availability breach only. If a backup is available, and data can be
restored in time, usually this would not be a breach that is likely to result in high risk to data subjects, and
a notification to data subjects would not be necessary. However, if the controller cannot restore the data
or the data has been copied, the consequences may be more significant, and notification to the SA, and
in some cases to individuals, may be necessary.

Ransomware attacks can be detected if appropriate detection systems are in place. However, even if the
attack is not detected, this does not mean the controller can escape its notification obligations. The
controller is expected to have detection systems in place.

If notification to individuals is required, the controller should be careful not to comply with any request
for a ransom payment, as this does not necessarily lead to mitigation of harm for the affected individuals.
Additionally, paying a ransom only encourages such attacks.



34
     https://www.enisa.europa.eu/publications/dbn-severity

Adopted                                                                                                     34
Prevention and mitigation strategies should be defined and implemented. These include keeping
firmware, operating system, and application software up to date, and ensuring malware/ransomware
protection is in place and up to date. Most ransomware is delivered via phishing attacks, and so a training
programme to help employees become less susceptible to phishing and social engineering is also
essential. It is important that robust and effective backup procedures are in place, and that backed-up
data is stored separately from operational data. Controls such as segmentation can also help limit the
damage of an attack, and organisations should test their systems to confirm they are resilient to
ransomware attacks.


              b)    Data exfiltration attacks
Data exfiltration attacks occur where an external attacker gains access to systems and extracts data, or
where the attacker sends code to systems that allows data to be intercepted or copied. Compromised
data can be used in a variety of ways to cause harm to data subjects, from identity fraud to public
disclosure. Such attacks should be notified to the SA if the type of data compromised would enable
individuals to be identified. The controller should explain the risk and advise on steps that data subjects
may take to mitigate any adverse effects.


              c)    Internal human risk source
Data breaches can also occur when an employee of the controller deliberately accesses and misuses
personal data. This can be for many reasons and can lead to both confidentiality and integrity breaches.
For example:

     •    An employee views and discloses personal data to a third party for their own purposes, with
          intent to cause damage to the data subject or the controller;
     •    An employee views and discloses personal data to a competitor for financial gain; or
     •    An employee views and discloses personal data to a third party without financial gain.

Controllers should have policies and procedures in place to mitigate such threats. These could include
access controls that limit the number of employees who have access to certain personal data, logging
systems that report on access to personal data or that track anomalous events.

When evaluating breaches caused by an internal source, the controller should investigate:

     •    What type of personal data has been accessed?
     •    Who has personal data been disclosed to?
     •    What was the motivation of the individual, if this can be ascertained?
     •    Were any mitigating factors in place that might have prevented disclosure of personal data, such
          as encryption?
     •    Is there any likelihood that the disclosed data could be further disseminated (for example, where
          data is disclosed online)?

If the personal data is sensitive, and the disclosure would be likely to result in significant harm to the data
subject, notification will be required to the SA and the data subjects. The type of personal data involved
will be relevant to the assessment (e.g. financial, health, location data, communications data).




Adopted                                                                                                    35
              d)   Lost or stolen devices and paper documents
A stolen (or lost) laptop or similar portable device, or a stolen paper document, if unencrypted or
unprotected, can be considered a breach. In this case, it is likely that confidential information may be
accessed or misused. In the case of an encrypted device, if the encryption is of a sufficient standard, in
most cases the data should not be considered to be at risk. Regardless of the encryption, if a backup exists
and it is possible to restore the data in a timely manner, the breach may be considered as a temporary
loss of availability.


              e)   Mis-delivery
A "mis-delivery" breach occurs when communications are sent to an incorrect recipient, either by
traditional post or electronic means. This could occur due to human or technical error, and could include
sending emails to the wrong person or sending documents by post to the wrong address. This includes
emails containing personal data that have been sent to the wrong person, or including more than one
intended recipient in the same communication. The controller should review the type of personal data
that has been sent to the wrong address and the nature of the recipient before deciding whether a breach
needs to be notified, and to whom. Notification to the SA will be required if the type of data could pose a
risk to the data subject. Notification to data subjects will also be necessary if the type of personal data
could pose a high risk to data subjects.


              f)   Social engineering
Social engineering includes phishing attacks where an employee is manipulated into divulging access
credentials, for example by clicking on malicious links within fake emails. Attackers can then potentially
access systems and exfiltrate data. Social engineering also includes manipulation to persuade an
employee to share data with a party that the employee believes to be the legitimate data subject but who
is in fact impersonating the data subject. Consequences will depend on what was disclosed and to whom.


              g)   Other
This category includes breaches that do not fit into any of the specific categories above. While it may be
possible to outline the particular types of breaches that commonly occur and that controllers must
address, it is not possible to create an exhaustive list. Controllers should be prepared for breaches that
have not been seen before or that cannot be anticipated in advance. A robust breach response plan will
assist in evaluating whether to notify the breach to the SA and/or the data subjects, and what information
should be included.




Adopted                                                                                                 36
