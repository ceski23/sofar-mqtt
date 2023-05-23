## Sensors

### Current power
Topic: `homeassistant/sensor/sofar/current_power/config`

Payload:
```json
{
  "~": "sofar",
  "name": "Current power",
  "unique_id": "sofar_current_power",
  "object_id": "sofar_current_power",
  "qos": 0,
  "unit_of_measurement": "W",
  "state_topic": "~/currentPower",
  "state_class": "measurement",
  "device_class": "power",
  "device": {
    "configuration_url": "http://10.0.0.64/index_cn.html",
    "identifiers": "sofar",
    "manufacturer": "Sofar",
    "model": "SF4ES003",
    "name": "Sofar SF4ES003",
    "sw_version": "LSW3_14_FFFF_1.0.34"
  }
}
```

### Inventer temperature
Topic: `homeassistant/sensor/sofar/inventer_temperature/config`

Payload:
```json
{
  "~": "sofar",
  "name": "Inventer temperature",
  "unique_id": "sofar_inventer_temperature",
  "object_id": "sofar_inventer_temperature",
  "qos": 0,
  "unit_of_measurement": "°C",
  "state_topic": "~/inverterTemp",
  "state_class": "measurement",
  "device_class": "temperature",
  "device": {
    "configuration_url": "http://10.0.0.64/index_cn.html",
    "identifiers": "sofar",
    "manufacturer": "Sofar",
    "model": "SF4ES003",
    "name": "Sofar SF4ES003",
    "sw_version": "LSW3_14_FFFF_1.0.34"
  }
}
```

### Yield today
Topic: `homeassistant/sensor/sofar/yield_today/config`

Payload:
```json
{
  "~": "sofar",
  "name": "Yield today",
  "unique_id": "sofar_yield_today",
  "object_id": "sofar_yield_today",
  "qos": 0,
  "unit_of_measurement": "kWh",
  "state_topic": "~/eToday",
  "state_class": "total_increasing",
  "device_class": "energy",
  "device": {
    "configuration_url": "http://10.0.0.64/index_cn.html",
    "identifiers": "sofar",
    "manufacturer": "Sofar",
    "model": "SF4ES003",
    "name": "Sofar SF4ES003",
    "sw_version": "LSW3_14_FFFF_1.0.34"
  }
}
```

### Yield total
Topic: `homeassistant/sensor/sofar/yield_total/config`

Payload:
```json
{
  "~": "sofar",
  "name": "Yield total",
  "unique_id": "sofar_yield_total",
  "object_id": "sofar_yield_total",
  "qos": 0,
  "unit_of_measurement": "kWh",
  "state_topic": "~/eTotal",
  "state_class": "total_increasing",
  "device_class": "energy",
  "device": {
    "configuration_url": "http://10.0.0.64/index_cn.html",
    "identifiers": "sofar",
    "manufacturer": "Sofar",
    "model": "SF4ES003",
    "name": "Sofar SF4ES003",
    "sw_version": "LSW3_14_FFFF_1.0.34"
  }
}
```




## sample data
Server response: `[165,10,0,16,23,23,23,79,172,254,103,0,1,89,129,82,100,120,0,0,0,200,21]`

Heartbeat: `[165,1,0,16,71,31,32,79,172,254,103,0,247,21]`

Data: `[165,151,0,16,66,44,45,79,172,254,103,1,1,39,173,118,1,0,27,12,0,0,228,17,81,100,1,0,187,1,0,0,83,70,52,69,83,48,48,51,77,52,67,48,53,56,32,32,114,1,94,11,217,2,5,0,0,0,8,0,9,0,8,0,225,8,207,8,213,8,134,19,110,0,0,0,229,1,0,0,176,119,0,0,246,23,0,0,2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,86,50,56,48,86,49,48,48,21,0,55,24,79,11,202,2,60,0,1,0,52,5,77,6,23,5,7,0,0,0,0,0,6,0,228,3,224,3,227,3,86,50,56,48,86,50,56,48,23,5,3,18,14,19,15,0,0,0,239,21]`

Data decoded:
```json
{
    "loggerSN": 1744743503,
    "sensorTypeList": "2701",
    "tOperationTime": 95917,
    "inverterSN": "SF4ES003M4C058",
    "inverterTemp": 37,
    "VDC1": 291,
    "VDC2": 72.9,
    "IDC1": 0.5,
    "IDC2": 0,
    "IAC1": 0.8,
    "IAC2": 0.9,
    "IAC3": 0.8,
    "VAC1": 227.3,
    "VAC2": 225.5,
    "VAC3": 226.1,
    "fAC": 49.98,
    "currentPower": 110,
    "eToday": 4.85,
    "eTotal": 3064,
    "hTotal": 6134,
    "inverterStatus": "normal",
    "loggerTemp": 21,
    "Vbus": 619.9,
    "VCPU1": 289.5,
    "countdownTime": 60,
    "PV1insulationResistance": 1332,
    "PV2insulationResistance": 1613,
    "cathode_groundInsulationImpedance": 1303,
    "countryCode": 7,
    "A_phaseDCdistribution": 996,
    "B_phaseDCdistribution": 992,
    "C_phaseDCdistribution": 995,
    "firmware": "V280",
    "year": 23,
    "month": 5,
    "day": 3,
    "hour": 18,
    "minute": 14,
    "second": 19
}
```

Server response 1: `[165,10,0,16,23,62,62,79,172,254,103,0,1,5,84,102,100,120,0,0,0,169,21]`
Server response 2: `[165,10,0,16,18,63,63,79,172,254,103,1,1,126,84,102,100,120,0,0,0,32,21]`




hello: `[165,86,0,16,65,0,1,79,172,254,103,2,36,35,14,0,33,0,0,0,0,0,0,0,5,60,120,2,37,1,76,83,87,51,95,49,52,95,70,70,70,70,95,49,46,48,46,51,52,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,52,234,231,44,60,22,49,48,46,48,46,48,46,54,52,0,0,0,0,0,0,0,1,0,1,1,39,170,21]`
decoded:
```json
{"loggerSN":1744743503,"tOperationTime":926500,"uploadingFrequency":5,"dataLoggingFrequency":60,"heartbeatFrequency":120,"commandType":2,"signalQuality":37,"sensorTypeNr":1,"moduleVersion":"LSW3_14_FFFF_1.0.34\u0000\u0000\u0000\u0000\u0000\u0000\u0000\u0000\u0000\u0000\u0000\u0000\u0000\u0000\u0000\u0000\u0000\u0000\u0000\u0000\u0000","macAddress":"34:ea:e7:2c:3c:16","localIP":"10.0.0.64\u0000\u0000\u0000\u0000\u0000\u0000","sensorTypeList":"2701"}
```

hello_cd: `[165,28,0,16,72,3,4,79,172,254,103,1,41,35,14,0,38,0,0,0,212,49,88,100,1,1,12,6,35,14,0,0,0,0,0,0,0,0,0,98,21]`
decoded:
```json
{"loggerSN":1744743503,"totalOperationTime":926505}
```

hello_end: `[165,60,0,16,72,7,8,79,172,254,103,1,46,35,14,0,43,0,0,0,212,49,88,100,1,5,44,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,85,21]`
decoded:
```json
{"loggerSN":1744743503,"totalOperationTime":926510}
```


https://developers.home-assistant.io/docs/core/entity/sensor






HELLO: `[165,86,0,16,65,3,4,79,172,254,103,2,71,125,14,0,127,0,0,0,0,0,0,0,5,60,120,2,25,1,76,83,87,51,95,49,52,95,70,70,70,70,95,49,46,48,46,51,52,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,52,234,231,44,60,22,49,48,46,48,46,48,46,54,52,0,0,0,0,0,0,0,1,0,1,1,39,127,21]`
RESPONSE: `[165,10,0,16,17,4,4,79,172,254,103,2,1,140,39,103,100,120,0,0,0,140,21]`



DATA: `[165,151,0,16,66,4,5,79,172,254,103,1,1,39,72,125,14,0,128,0,0,0,69,170,88,100,1,0,40,13,0,0,83,70,52,69,83,48,48,51,77,52,67,48,53,56,32,32,104,1,122,11,213,2,12,0,0,0,9,0,10,0,9,0,195,8,216,8,201,8,135,19,54,1,0,0,69,0,0,0,174,126,0,0,220,24,0,0,2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,86,50,56,48,86,49,48,48,21,0,4,24,100,11,193,2,60,0,1,0,40,5,87,6,33,5,7,0,0,0,0,0,6,0,226,3,227,3,227,3,86,50,56,48,86,50,56,48,23,5,19,9,36,49,37,0,0,0,96,21]`
RESPONSE: `[165,10,0,16,18,5,5,79,172,254,103,1,1,141,39,103,100,120,0,0,0,143,21]`

HELLO_END: `[165,60,0,16,72,9,13,79,172,254,103,1,194,133,14,0,139,0,0,0,110,170,88,100,1,5,44,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,197,21]`
RESPONSE: `[165,10,0,16,24,10,13,79,172,254,103,1,1,48,48,103,100,120,0,0,0,78,21]`