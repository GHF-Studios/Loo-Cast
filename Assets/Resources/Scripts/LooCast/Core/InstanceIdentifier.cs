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
        public string PrefabPath => prefabPath;
        public string UUID
        {
            get
            {
                string uuid = ID + $"_{instanceID}";
                return uuid;
            }
        }               // Example: Enemy_SmolEnemy_69

        [SerializeField] protected int instanceID;
        [SerializeField] protected string prefabPath;

        public InstanceIdentifier(int instanceID, Type type, string prefabPath) : base(type)
        {
            this.instanceID = instanceID;
            this.prefabPath = prefabPath;
        }

        public override string ToString()
        {
            return UUID;
        }
    }
}
