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






https://developers.home-assistant.io/docs/core/entity/sensor
