class ActuatorData {
    client: string;
    clientDeviceId: string;
    clientName: string;
    editable: number;
    id: string;
    ignored: number;
    metadataHash: string;
    methods: number;
    name: string;
    online: string;
    parametersHash: string;
    state: number;
    stateValues: any[];
    statevalue: string;
    type: string;
  
    constructor(data: {
      client: string;
      clientDeviceId: string;
      clientName: string;
      editable: number;
      id: string;
      ignored: number;
      metadataHash: string;
      methods: number;
      name: string;
      online: string;
      parametersHash: string;
      state: number;
      stateValues: any[];
      statevalue: string;
      type: string;
    }) {
      this.client = data.client;
      this.clientDeviceId = data.clientDeviceId;
      this.clientName = data.clientName;
      this.editable = data.editable;
      this.id = data.id;
      this.ignored = data.ignored;
      this.metadataHash = data.metadataHash;
      this.methods = data.methods;
      this.name = data.name;
      this.online = data.online;
      this.parametersHash = data.parametersHash;
      this.state = data.state;
      this.stateValues = data.stateValues;
      this.statevalue = data.statevalue;
      this.type = data.type;
    }
  }
  