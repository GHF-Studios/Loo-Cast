using System;
using UnityEngine;

namespace LooCast.Core
{
    /// <summary>
    /// Uniquely identifies a Type, which inherits from ExtendedMonoBehaviour, and one GameObject instance of it, and the corresponding File that holds that GameObject's RuntimeData
    /// </summary>
    [Serializable]
    public class InstanceIdentifier : Identifier
    {
        public int InstanceID => instanceID;
        public string UUID          // Example: Enemy_SmolEnemy_69 
        {
            get
            {
                string uuid = ID + $"_{instanceID}";
                return uuid;
            }
        }
        

        [SerializeField] protected int instanceID;

        public InstanceIdentifier(int instanceID, Type type, string prefabPath, params string[] directoryNames) : base(type, prefabPath, directoryNames)
        {
            this.instanceID = instanceID;
        }

        public override string ToString()
        {
            return UUID;
        }
    }
}
