# Guidelines 01/2021 on Examples regarding Personal Data Breach Notification

> Source: https://www.edpb.europa.eu/system/files/2022-01/edpb_guidelines_012021_pdbnotification_adopted_en.pdf
> Version: 2.0
> Language: EN

---

                                      Guidelines 01/2021
         on Examples regarding Personal Data Breach
                        Notification
                              Adopted on 14 December 2021
                                          Version 2.0




                                                            1
Adopted - after public consultation
Version history


 Version 2.0     14 12 2021           Adoption of the Guidelines after public consultation

 Version 1.0     14 01 2021           Adoption of the Guidelines for public consultation




                                                                                             2
Adopted - after public consultation
Table of contents
1     INTRODUCTION........................................................................................................................................5
2     RANSOMWARE.........................................................................................................................................8
    2.1      CASE No. 01: Ransomware with proper backup and without exfiltration.......................................8
      2.1.1         CASE No. 01 - Prior measures and risk assessment .................................................................8
      2.1.2         CASE No. 01 - Mitigation and obligations ...............................................................................9
    2.2      CASE No. 02: Ransomware without proper backup ......................................................................10
      2.2.1         CASE No. 02 - Prior measures and risk assessment ...............................................................10
      2.2.2         CASE No. 02 - Mitigation and obligations .............................................................................11
    2.3      CASE No. 03: Ransomware with backup and without exfiltration in a hospital ............................12
      2.3.1         CASE No. 03 - Prior measures and risk assessment ...............................................................12
      2.3.2         CASE No. 03 - Mitigation and obligations .............................................................................12
    2.4      CASE No. 04: Ransomware without backup and with exfiltration.................................................13
      2.4.1         CASE No. 04 - Prior measures and risk assessment ...............................................................13
      2.4.2         CASE No. 04 - Mitigation and obligations .............................................................................14
    2.5    Organizational and technical measures for preventing / mitigating the impacts of ransomware
    attacks14
3     Data Exfiltration ATTACKS......................................................................................................................15
    3.1      CASE No. 05: Exfiltration of job application data from a website .................................................15
      3.1.1         CASE No. 05 - Prior measures and risk assessment ...............................................................15
      3.1.2         CASE No. 05 - Mitigation and obligations .............................................................................16
    3.2      CASE No. 06: Exfiltration of hashed password from a website .....................................................17
      3.2.1         CASE No. 06 - Prior measures and risk assessment ...............................................................17
      3.2.2         CASE No. 06 - Mitigation and obligations .............................................................................17
    3.3      CASE No. 07: Credential stuffing attack on a banking website......................................................18
      3.3.1         CASE No. 07 - Prior measures and risk assessment ...............................................................18
      3.3.2         CASE No. 07 - Mitigation and obligations .............................................................................18
    3.4    Organizational and technical measures for preventing / mitigating the impacts of hacker
    attacks19
4     INTERNAL HUMAN RISK SOURCE...........................................................................................................20
    4.1      CASE No. 08: Exfiltration of business data by an employee ..........................................................20
      4.1.1         CASE No. 08 - Prior measures and risk assessment ...............................................................20
      4.1.2         CASE No. 08 - Mitigation and obligations .............................................................................21
    4.2      CASE No. 09: Accidental transmission of data to a trusted third party.........................................22
      4.2.1         CASE No. 09 - Prior measures and risk assessment ..............................................................22
      4.2.2         CASE No. 09 - Mitigation and obligations .............................................................................22

                                                                                                                                                         3
Adopted - after public consultation
    4.3   Organizational and technical measures for preventing / mitigating the impacts of internal
    human risk sources ....................................................................................................................................22
5      LOST OR STOLEN DEVICES AND PAPER DOCUMENTS............................................................................23
    5.1        CASE No. 10: Stolen material storing encrypted personal data ....................................................24
       5.1.1           CASE No. 10 - Prior measures and risk assessment ...............................................................24
       5.1.2           CASE No. 10 - Mitigation and obligations .............................................................................24
    5.2        CASE No. 11: Stolen material storing non-encrypted personal data.............................................25
       5.2.1           CASE No. 11 - Prior measures and risk assessment ...............................................................25
       5.2.2           CASE No. 11 - Mitigation and obligations .............................................................................25
    5.3        CASE No. 12: Stolen paper files with sensitive data ......................................................................25
       5.3.1           CASE No. 12 - Prior measures and risk assessment ..............................................................26
       5.3.2           CASE No. 12 - Mitigation and obligations .............................................................................26
    5.4    Organizational and technical measures for preventing / mitigating the impacts of loss or theft of
    devices .......................................................................................................................................................26
6      MISPOSTAL.............................................................................................................................................27
    6.1        CASE No. 13: Postal mail mistake ..................................................................................................27
       6.1.1           CASE No. 13 - Prior measures and risk assessment ...............................................................27
       6.1.2           CASE No. 13 - Mitigation and obligations .............................................................................27
    6.2        CASE No. 14: Highly confidential personal data sent by mail by mistake .....................................28
       6.2.1           CASE No. 14 - Prior measures and risk assessment ...............................................................28
       6.2.2           CASE No. 14 - Mitigation and obligations .............................................................................28
    6.3        CASE No. 15: Personal data sent by mail by mistake.....................................................................28
       6.3.1           CASE No. 15 - Prior measures and risk assessment ...............................................................28
       6.3.2           CASE No. 15 - Mitigation and obligations .............................................................................29
    6.4        CASE No. 16: Postal mail mistake ..................................................................................................29
       6.4.1           CASE No. 16 - Prior measures and risk assessment ...............................................................29
       6.4.2           CASE No. 16 - Mitigation and obligations .............................................................................30
    6.5        Organizational and technical measures for preventing / mitigating the impacts of mispostal ....30
7      Other Cases - Social Engineering...........................................................................................................31
    7.1        CASE No. 17: Identity theft ............................................................................................................31
       7.1.1           CASE No. 17 - Risk assessment, mitigation and obligations ..................................................31
    7.2        CASE No. 18: Email exfiltration ......................................................................................................32
       7.2.1           CASE No. 18 - Risk assessment, mitigation and obligations ..................................................32




                                                                                                                                                                   4
Adopted - after public consultation
     THE EUROPEAN DATA PROTECTION BOARD

     Having regard to Article 70 (1e) of the Regulation 2016/679/EU of the European Parliament and of the
     Council of 27 April 2016 on the protection of natural persons with regard to the processing of personal data
     and on the free movement of such data, and repealing Directive 95/46/EC, (hereinafter "GDPR"),

     Having regard to the EEA Agreement and in particular to Annex XI and Protocol 37 thereof, as amended by
     the Decision of the EEA joint Committee No 154/2018 of 6 July 20181,

     Having regard to Article 12 and Article 22 of its Rules of Procedure,

     Having regard to the Communication from the Commission to the European Parliament and the Council
     titled Data protection as a pillar of citizens' empowerment and the EU's approach to the digital transition -
     two years of application of the General Data Protection Regulation2,


     HAS ADOPTED THE FOLLOWING GUIDELINES


     1 INTRODUCTION
1.   The GDPR introduces, in certain cases, the requirement for a personal data breach to be notified to the
     competent national supervisory authority (hereinafter "SA") and to communicate the breach to the
     individuals whose personal data have been affected by the breach (Articles 33 and 34).

2.   The Article 29 Working Party already produced a general guidance on data breach notification in October
     2017, analysing the relevant Sections of the GDPR (Guidelines on Personal data breach notification under
     Regulation 2016/679, WP 250) (hereinafter "Guidelines WP250)3. However, due to its nature and timing,
     this guideline did not address all practical issues in sufficient detail. Therefore, the need has arisen for a
     practice-oriented, case-based guidance, that utilizes the experiences gained by SAs since the GDPR is
     applicable.

3.   This document is intended to complement the Guidelines WP 250 and it reflects the common experiences
     of the SAs of the EEA since the GDPR became applicable. Its aim is to help data controllers in deciding how
     to handle data breaches and what factors to consider during risk assessment.

4.   As part of any attempt to address a breach the controller and processor should first be able to recognize
     one. The GDPR defines a "personal data breach" in Article 4(12) as "a breach of security leading to the




     1
      References to "Member States" made throughout this document should be understood as references to "EEA
     Member States".
     2
         COM(2020) 264 final, 24 June 2020.

     3
      G29 WP250 rev.1, 6 February 2018, Guidelines on Personal data breach notification under Regulation 2016/679 -
     endorsed by the EDPB, https://ec.europa.eu/newsroom/article29/item-detail.cfm?item_id=612052.




                                                                                                                      5
     Adopted - after public consultation
     accidental or unlawful destruction, loss, alteration, unauthorised disclosure of, or access to, personal data
     transmitted, stored or otherwise processed".

5.   In its Opinion 03/2014 on breach notification4 and in its Guidelines WP 250, WP29 explained that breaches
     can be categorised according to the following three well-known information security principles:

        "Confidentiality breach" - where there is an unauthorised or accidental disclosure of, or access to,
         personal data.
      "Integrity breach" - where there is an unauthorised or accidental alteration of personal data.
      "Availability breach" - where there is an accidental or unauthorised loss of access to, or destruction of,
         personal data.5
6.   A breach can potentially have a range of significant adverse effects on individuals, which can result in
     physical, material, or non-material damage. The GDPR explains that this can include loss of control over
     their personal data, limitation of their rights, discrimination, identity theft or fraud, financial loss,
     unauthorised reversal of pseudonymisation, damage to reputation, and loss of confidentiality of personal
     data protected by professional secrecy. It can also include any other significant economic or social
     disadvantage to those individuals. One of the most important obligation of the data controller is to evaluate
     these risks to the rights and freedoms of data subjects and to implement appropriate technical and
     organizational measures to address them.

7.   Accordingly, the GDPR requires the controller to:

        document any personal data breaches, comprising the facts relating to the personal data breach, its
         effects and the remedial action taken6;
      notify the personal data breach to the supervisory authority, unless the data breach is unlikely to result
         in a risk to the rights and freedoms of natural persons7;
      communicate the personal data breach to the data subject when the personal data breach is likely to
         result in a high risk to the rights and freedoms of natural persons8.
8.   Data breaches are problems in and of themselves, but they may be also symptoms of a vulnerable, possibly
     outdated data security regime, they may also indicate system weaknesses to be addressed. As a general
     truth, it is always better to prevent data breaches by preparing in advance, since several consequences of
     them are by nature irreversible. Before a controller can fully assess the risk arising from a breach caused by
     some form of attack, the root cause of the issue should be identified, in order to identify whether any
     vulnerabilities that gave rise to the incident are still present, and are still therefore exploitable. In many
     cases the controller is able to identify that the incident is likely to result in a risk, and is therefore to be




     4
      G29 WP213, 25 March 2014, Opinion 03/2014 on Personal Data Breach Notification, p. 5,
     https://ec.europa.eu/justice/article-29/documentation/opinion-recommendation/index_en.htm#maincontentSec4 .
     5
      See Guidelines WP 250, p. 7. - It must be taken into consideration that a data breach can concern either one
     category or more categories simultaneously or combined.
     6
         GDPR Article 33(5).
     7
         GDPR Article 33(1).
     8
         GDPR Article 34(1).




                                                                                                                     6
     Adopted - after public consultation
      notified. In other cases the notification does not need to be postponed until the risk and impact surrounding
      the breach has been fully assessed, since the full risk assessment can happen in parallel to notification, and
      the information thus gained may be provided to the SA in phases without undue further delay9.

9.    The breach should be notified when the controller is of the opinion that it is likely to result in a risk to the
      rights and freedoms of the data subject. Controllers should make this assessment at the time they become
      aware of the breach. The controller should not wait for a detailed forensic examination and (early)
      mitigation steps before assessing whether or not the data breach is likely to result in a risk and thus should
      be notified.

10.   If a controller self-assesses the risk to be unlikely, but it turns out that the risk materializes, the competent
      SA can use its corrective powers and may resolve to sanctions

11.   Every controller and processor should have plans, procedures in place for handling eventual data breaches.
      Organisations should have clear reporting lines and persons responsible for certain aspects of the recovery
      process

12.   Training and awareness on data protection issues for the staff of the controller and processor focusing on
      personal data breach management (identification of a personal data breach incident and further actions to
      be taken, etc.) is also essential for the controllers and processors. This training should be regularly repeated,
      depending on the type of the processing activity and size of the controller, addressing latest trends and
      alerts coming from cyberattacks or other security incidents.

13.   The principle of accountability and the concept of data protection by design could incorporate analysis that
      feeds into a data controller's and data processor's own "Handbook on Handling Personal Data Breach" that
      aims to establish facts for each facet of the processing at each major stage of the operation. Such a
      handbook prepared in advance would provide a much quicker source of information to allow data
      controllers and data processors to mitigate the risks and meet the obligations without undue delay. This
      would ensure that if a personal data breach was to occur, people in the organisation would know what to
      do, and the incident would more than likely be handled quicker than if there were no mitigations or plan in
      place.

14.   Though the cases presented below are fictitious, they are based on typical cases from the SA's collective
      experience with data breach notifications. The analyses offered relate explicitly to the cases under scrutiny,
      but with the goal to provide assistance for data controllers in assessing their own data breaches. Any
      modification in the circumstances of the cases described below may result in different or more significant
      levels of risk, thus requiring different or additional measures. These guidelines structure the cases according
      to certain categories of breaches (e.g. ransomware attacks). Certain mitigating measures are called for in
      each case when dealing with a certain category of breaches. These measures are not necessarily repeated
      in each case analysis belonging to the same category of breaches. For the cases belonging to the same
      category only the differences are laid out. Therefore, the reader should read all cases relevant to relevant
      category of a breach to identify and distinguish all the correct measures to be taken.

15.   The internal documentation of a breach is an obligation independent of the risks pertaining to the breach,
      and must be performed in each and every case. The cases presented below try to shed some light on
      whether or not to notify the breach to the SA and communicate it to the data subjects affected.




      9
          GDPR Article 33(4).


                                                                                                                     7
      Adopted - after public consultation
      2 RANSOMWARE
16.   A frequent cause for a data breach notification is a ransomware attack suffered by the data controller. In
      these cases a malicious code encrypts the personal data, and subsequently the attacker asks the controller
      for a ransom in exchange for the decryption code. This kind of attack can usually be classified as a breach
      of availability, but often also a breach of confidentiality could occur.

      2.1 CASE No. 01: Ransomware with proper backup and without exfiltration
       Computer systems of a small manufacturing company were exposed to a ransomware attack, and
       data stored in those systems was encrypted. The data controller used encryption at rest, so all data
       accessed by the ransomware was stored in encrypted form using a state-of-the-art encryption
       algorithm. The decryption key was not compromised in the attack, i.e. the attacker could neither
       access it nor use it indirectly. In consequence, the attacker only had access to encrypted personal
       data. In particular, neither the email system of the company, nor any client systems used to access it
       were affected. The company is using the expertise of an external cybersecurity company to
       investigate the incident. Logs tracing all data flows leaving the company (including outbound email)
       are available. After analysing the logs and the data collected by the detection systems the company
       has deployed, an internal investigation supported by the external cybersecurity company determined
       with certainty that the perpetrator only encrypted data, without exfiltrating it. The logs show no
       outward data flow in the timeframe of the attack. The personal data affected by the breach relates
       to clients and employees of the company, a few dozen individuals altogether. A backup was readily
       available, and the data was restored a few hours after the attack took place. The breach did not result
       in any consequences on the day-to-day operation of the controller. There was no delay in employee
       payments or handling client requests.
17.   In this case, the following elements were realized from the definition of a 'personal data breach': a breach
      of security led to unlawful alteration and unauthorized access to personal data stored.

      2.1.1 CASE No. 01 - Prior measures and risk assessment
18.   As with all risks posed by external actors, the likelihood that a ransomware attack is successful can be
      drastically reduced by tightening the security of the data controlling environment. The majority of these
      breaches can be prevented by ensuring that appropriate organizational, physical and technological security
      measures have been taken. Examples of such measures are proper patch management and the use of an
      appropriate anti-malware detection system. Having a proper and separate backup will help to mitigate the
      consequences of a successful attack should it occur. Moreover, an employee security education, training,
      and awareness (SETA) program, will help to prevent and recognise this kind of attack. (A list of advisable
      measures can be found in section 2.5.) Among those measures, a proper patch management that ensures
      that the systems are up to date and all known vulnerabilities of the deployed systems are fixed is one of
      the most important since most of the ransomware attacks exploit well known vulnerabilities.

19.   When assessing the risks, the controller should investigate the breach and identify the type of the malicious
      code to understand the possible consequences of the attack. Among those risks to be considered is the risk
      that data was exfiltrated without leaving a trace in the logs of the systems.

20.   In this example, the attacker had access to personal data and the confidentiality of cipher text containing
      personal data in encrypted form was compromised. However, any data that might have been exfiltrated
      cannot be read or used by the perpetrator, at least for the time being. The encryption technique used by
      the data controller conforms to the state-of-the-art. The decryption key was not compromised and
      presumably could also not be determined by other means. In consequence, the confidentiality risks to the


                                                                                                                 8
      Adopted - after public consultation
      rights and freedoms of natural persons are reduced to a minimum barring cryptanalytic progress that
      renders the encrypted data intelligible in the future.

21.   The data controller should consider the risk to individuals due to the breach10. In this case, it appears the
      risks to the rights and freedoms of data subjects result from the lack of availability of the personal data, and
      the confidentiality of the personal data is not compromised11. In this example, the adverse effects of the
      breach were mitigated fairly soon after the breach occurred. Having a proper backup regime12 makes the
      effects of the breach less severe and here the controller was able to effectively make use of it.

22.   On the severity of the consequences for the data subjects, only minor consequences could be identified
      since the affected data was restored in a few hours, the breach did not result in any consequences on the
      day-to-day operation of the controller and had no significant effect on the data subjects (e.g. employee
      payments or handling client requests).

      2.1.2 CASE No. 01 - Mitigation and obligations
23.   Without a backup few measures to remediate the loss of personal data can be undertaken by the controller,
      and the data has to be collected again. In this particular case however, the impacts of the attack could
      effectively be contained by resetting all compromised systems to a clean state known to be free of malicious
      code, fixing the vulnerabilities and restoring the affected data soon after the attack. Without a backup, data
      is lost and the severity may increase because risks or impacts to individuals may also do so.

24.   The timeliness of an effective data restoration from the readily available backup is a key variable when
      analysing the breach. Specifying an appropriate timeframe to restore the compromised data depends on
      the unique circumstances of the breach at hand. The GDPR states that a personal data breach shall be
      notified without undue delay and, where feasible, not later than after 72 hours. Therefore, it could be
      determined that exceeding the 72-hour time limit is unadvisable in any case, but when dealing with high
      risk level cases, even complying with this deadline can be viewed as unsatisfactory.

25.   In this case, following a detailed impact assessment and incident response process, the controller
      determined that the breach was unlikely to result in a risk to the rights and freedoms of natural persons,
      hence no communication to the data subjects is necessary, nor does the breach require a notification to




      10
        For guidance on "likely to result in high risk" processing operations, see A29 Working Party "Guidelines on Data
      Protection Impact Assessment (DPIA) and determining whether processing is "likely to result in a high risk" for the
      purposes of Regulation 2016/679", WP248 rev. 01, - endorsed by EDPB,
      https://ec.europa.eu/newsroom/article29/items/611236, p. 9.
      11
        Technically, encryption of data will involve "access" to original data, and in the case of ransomware, the deletion of
      the original - the data needs to be accessed by ransomware code to encrypt it, and to remove the original data. An
      attacker may take a copy of the original before deletion, but personal data will not always be extracted. As a data
      controller's investigation progresses, new information may come to light to make this assessment change. Access that
      results in unlawful destruction, loss, alteration, unauthorised disclosure of the personal data, or to a security risk to a
      data subject, even without interpretation of the data may be as severe as access with interpretation of the personal
      data.
      12
        Backup procedures should be structured, consistent and repeatable. Examples of back up procedures are the 3-2-1
      method and the grandfather-father-son method. Any method should always be tested for effectiveness in coverage
      and when data is to be restored. Testing should also be repeated at intervals and especially when changes occur in
      the processing operation or its circumstances to ensure the integrity of the system.


                                                                                                                               9
      Adopted - after public consultation
      the SA. However, as all data breaches, it should be documented in accordance with Article 33 (5). The
      organisation may also need (or later be required by the SA) to update and remediate its organizational and
      technical personal data security handling and risk mitigation measures and procedures. Within the frame
      of this update and remediation, the organisation should thoroughly investigate the breach and identify the
      causes and the methods used by the perpetrator in order to prevent any similar events in the future.

                                          Actions necessary based on the identified risks
               Internal documentation                   Notification to SA             Communication to data subjects

                                                                 X                                        X

This document continues with additional case studies covering ransomware attacks without backup, hospital scenarios, data exfiltration attacks, internal human risk sources, lost/stolen devices, mispostal incidents, and social engineering attacks. Each case provides detailed guidance on prior measures, risk assessment, mitigation strategies, and notification obligations under the GDPR.
