using System;
using UnityEngine;

namespace LooCast.Game
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
        public InstanceIdentifier Identifier
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
                for (int i = 0; i < Identifier.DirectoryNames.Length; i++)
                {
                    dataFilePath += $"{Identifier.DirectoryNames[i]}/";
                }
                dataFilePath += $"{Identifier.Type.Name}/{Identifier.InstanceID}.json";
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
