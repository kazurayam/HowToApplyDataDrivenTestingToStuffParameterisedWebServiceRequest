import com.kms.katalon.core.testdata.TestData
import com.kms.katalon.core.testdata.TestDataFactory
import com.kms.katalon.core.testobject.RestRequestObjectBuilder
import com.kms.katalon.core.util.KeywordUtil as KeywordUtil
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS

import groovy.json.JsonOutput
import groovy.text.GStringTemplateEngine

// read values from a file at `Include/fixture/data.csv` 
TestData testData = TestDataFactory.findTestData("Values")
List<List<Object>> allData = testData.getAllData()

// prepare instace of GStringTemplateEngine.
def engine = new GStringTemplateEngine()

// the URL template
String source = 'https://${uri_code_user}.execute-api.${aws_region}.amazonaws.com/${env}/${aws_region}/usermanagement/users/${tenantId}'

// iterate over lines in the data.csv while interpolating varials with given values to build URLs to send request to
int rowNumber = testData.getRowNumbers()
for (int i = 1; i <= rowNumber; i++) {
	def binding = [
		uri_code_user:	testData.getObjectValue('uri_code_user', i),
		aws_region:		testData.getObjectValue('aws_region', i),
		env:			testData.getObjectValue('env', i),
		aws_region:		testData.getObjectValue('aws_region', i),
		tenantId:		testData.getObjectValue('tenantId', i)
		]
	String url = engine.createTemplate(source).make(binding).toString()
	KeywordUtil.logInfo(url)
	
	// How to build a web service request on the fly?
	// see https://docs.katalon.com/katalon-studio/docs/web-services-builder.html
	
	// Create a new GET object using builder'
	def builder = new RestRequestObjectBuilder()
	def requestObject = builder
		.withRestRequestMethod("GET")
		.withRestUrl(url)
		.build()
	// Send a request'
	//def response = WS.sendRequest(requestObject)
	
	// do whatever you like on the response
	//WS.verifyElementPropertyValue(response, '[0].email', 'Eliseo@gardner.biz')
	//KeywordUtil.logInfo(JsonOutput.prettyPrint(response))
}