<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GetAllUsersOfTenant</name>
   <tag></tag>
   <elementGuidId>511592e7-4d58-4b36-a76f-22dba516141c</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>https://${uri_code_user}.execute-api.${aws_region}.amazonaws.com/${env}/${aws_region}/usermanagement/users/${tenantId}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>be128cda-e68c-41d1-9c10-2c8492b06ba6</id>
      <masked>false</masked>
      <name>uri_code_user</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>9bae723b-3185-438c-9e9a-5802ad420dde</id>
      <masked>false</masked>
      <name>aws_region</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>d0740250-45bd-40f7-90c4-e8c3a6aa2183</id>
      <masked>false</masked>
      <name>env</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>c5376445-708e-414b-967e-602aa142e94d</id>
      <masked>false</masked>
      <name>tenantId</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
