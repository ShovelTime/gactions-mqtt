class TempSensorData {
    battery: number;
    client: string;
    clientName: string;
    editable: number;
    humidity: string;
    id: string;
    ignored: number;
    keepHistory: number;
    lastUpdated: number;
    miscValues: Record<string, any>;
    model: string;
    name: string;
    online: string;
    protocol: string;
    sensorId: string;
    temp: string;
  
    constructor(data: {
      battery: number;
      client: string;
      clientName: string;
      editable: number;
      humidity: string;
      id: string;
      ignored: number;
      keepHistory: number;
      lastUpdated: number;
      miscValues: Record<string, any>;
      model: string;
      name: string;
      online: string;
      protocol: string;
      sensorId: string;
      temp: string;
    }) {
      this.battery = data.battery;
      this.client = data.client;
      this.clientName = data.clientName;
      this.editable = data.editable;
      this.humidity = data.humidity;
      this.id = data.id;
      this.ignored = data.ignored;
      this.keepHistory = data.keepHistory;
      this.lastUpdated = data.lastUpdated;
      this.miscValues = data.miscValues;
      this.model = data.model;
      this.name = data.name;
      this.online = data.online;
      this.protocol = data.protocol;
      this.sensorId = data.sensorId;
      this.temp = data.temp;
    }
  }