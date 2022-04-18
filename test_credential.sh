./didkit generate-ed25519-key > issuer_key.jwk
issuer_did=$(./didkit key-to-did key -k issuer_key.jwk)
echo $issuer_did
cat > unsigned-vc.json <<EOF
{
    "@context": [ "https://www.w3.org/2018/credentials/v1", "https://credential.codegene.xyz/context/credential.jsonld" ],
    "id": "urn:uuid:`uuidgen`",
    "type": ["VerifiableCredential"],
    "issuer": "${issuer_did}",
    "issuanceDate": "$(date -u +%FT%TZ)",
    "credentialSubject": {
      "documentNumber": "authority issued identity number",
      "firstName": "",
      "lastName": "",
      "fullName": "",
      "sex": "",
      "dob": "yyyy/mm/dd",
      "address1": "",
      "documentType": "I",
      "issuerOrgFull": "China",
      "issuerOrgIso2": "CN",
      "issuerOrgIso3": "CHN",
      "nationalityFull": "China",
      "nationalityIso2": "CN",
      "nationalityIso3": "CHN",
      "expiry": "yyyy/mm/dd",
      "issued": "yyyy/mm/dd",
      "issueAuthority": "",
      "faceIsIdentical": true,
      "faceConfidence": "0.853",
      "verificationFace": true,
      "verificationDigit": false,
      "authenticationScore": "0.99",
      "resDocumentFront": "href address",
      "resDocumentBack": "href address",
      "resFaceImage": "href address",
      "resFaceVideo": "href address"
    }
}
EOF

vm=$(./didkit key-to-verification-method key --key-path issuer_key.jwk)
./didkit vc-issue-credential --key-path issuer_key.jwk \
                           -v "${vm}" -p assertionMethod \
                           <unsigned-vc.json > signed-vc.json
cat signed-vc.json
./didkit vc-verify-credential < signed-vc.json