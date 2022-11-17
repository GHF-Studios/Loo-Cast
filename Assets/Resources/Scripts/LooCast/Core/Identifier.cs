using System;
using System.Linq;
using UnityEngine;

namespace LooCast.Core
{
    /// <summary>
    /// Uniquely identifies a Type, which inherits from ExtendedMonoBehaviour
    /// </summary>
    [Serializable]
    public class Identifier
    {
        public Type Type
        {
            get
            {
                return Type.GetType(assemblyQualifiedName);
            }
        }                  // Example: HugeBoss
        public string[] Namespaces
        {
            get
            {
                string[] namespaces = Type.FullName.Split("LooCast.")[0].Split('.');
                namespaces = namespaces.Take(namespaces.Count() - 1).ToArray();
                return namespaces;
            }
        }       // Example: Enemy, Boss
        public string ID
        {
            get
            {
                string id = "";
                for (int i = 0; i < Namespaces.Length; i++)
                {
                    id += $"{Namespaces[i]}_";
                }
                id += $"{Type.Name}";
                return id;
            }
        }                // Example: Enemy_Boss_HugeBoss

        [SerializeField] private string assemblyQualifiedName;

        public Identifier(Type type)
        {
            assemblyQualifiedName = type.AssemblyQualifiedName;
        }

        public override string ToString()
        {
            return ID;
        }
    }
}
