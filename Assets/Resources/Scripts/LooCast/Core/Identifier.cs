using System;
using UnityEngine;

namespace LooCast.Core
{
    /// <summary>
    /// Uniquely identifies a Type, which inherits from ExtendedMonoBehaviour
    /// </summary>
    [Serializable]
    public class Identifier
    {
        public Type Type => type;
        public string PrefabPath => prefabPath;
        public string[] DirectoryNames => directoryNames;
        public string ID            // Example: Enemy_SmolEnemy, Health_EnemyHealth
        {
            get
            {
                string id = "";
                for (int i = 0; i < directoryNames.Length; i++)
                {
                    id += $"{directoryNames[i]}_";
                }
                id += $"{type.Name}";
                return id;
            }
        }

        [SerializeField] protected Type type;
        [SerializeField] protected string prefabPath;
        [SerializeField] protected string[] directoryNames;

        public Identifier(Type type, string prefabPath, params string[] directoryNames)
        {
            this.type = type;
            this.prefabPath = prefabPath;
            this.directoryNames = directoryNames;
        }


        public override string ToString()
        {
            return ID;
        }
    }
}
