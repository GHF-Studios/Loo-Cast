using System;
using UnityEngine;

namespace LooCast.Data.Runtime
{
    using Core;

    public struct RuntimeData
    {
        public string JsonSerializedData
        {
            get 
            {
                return jsonSerializedData;
            }
        }
        public InstanceIdentifier InstanceIdentifier
        {
            get
            {
                return instanceIdentifier;
            }
        }
        public string DataFilePath  // Example: Enemy/SmolEnemy/69.json
        {
            get
            {
                string dataFilePath = "";
                for (int i = 0; i < InstanceIdentifier.Namespaces.Length; i++)
                {
                    dataFilePath += $"{InstanceIdentifier.Namespaces[i]}/";
                }
                dataFilePath += $"{InstanceIdentifier.Type.Name}/{InstanceIdentifier.InstanceID}.json";
                return dataFilePath;
            }
        }

        [SerializeField] private string jsonSerializedData;
        [SerializeField] private InstanceIdentifier instanceIdentifier;

        public RuntimeData(string jsonSerializedData, InstanceIdentifier instanceIdentifier)
        {
            this.jsonSerializedData = jsonSerializedData;
            this.instanceIdentifier = instanceIdentifier;
        }
    }
}
