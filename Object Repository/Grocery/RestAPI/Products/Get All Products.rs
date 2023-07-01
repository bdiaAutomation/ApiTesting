<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>Get all products 
endpoint : /products</description>
   <name>Get All Products</name>
   <tag></tag>
   <elementGuidId>9e67bd3e-fd35-4bb5-856d-9caca9b3bf02</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <katalonVersion>8.5.5</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${GlobalVariable.groceryUri}/products</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <validationSteps>
      <id>fe46f14b-39cd-43b0-92da-c78bd85974ef</id>
      <name>Response format</name>
      <type>JSON_SCHEMA</type>
      <dataType>FILE</dataType>
      <target>RESPONSE</target>
      <data>/Users/bambadia/IdeaProjects/DummyCucumber/src/test/java/mypackage/ResponseSchema.json</data>
      <activate>true</activate>
   </validationSteps>
   <validationSteps>
      <id>39961339-0e7d-4100-a56a-a4920ca79f3e</id>
      <name>Request Format</name>
      <type>JSON_SCHEMA</type>
      <dataType>FILE</dataType>
      <target>REQUEST</target>
      <data>/Users/bambadia/IdeaProjects/DummyCucumber/requestSchema.json</data>
      <activate>true</activate>
   </validationSteps>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()


WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)


def jsonSlurper = new JsonSlurper()

def jsonResponse = jsonSlurper.parseText(response.getResponseText())

assertThat(jsonResponse[0].category.class).isEqualTo(String)
assertThat(jsonResponse[0].id.class).isEqualTo(Integer)



assertThat(jsonResponse[0].category).isEqualTo(&quot;coffee&quot;)
WS.verifyElementPropertyValue(response, '[0].category', &quot;coffee&quot;)
println('ARRAY SIZE IS : ' + jsonResponse.size())

</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
