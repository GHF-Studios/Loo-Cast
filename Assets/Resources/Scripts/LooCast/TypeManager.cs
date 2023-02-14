using System;
using UnityEngine;

namespace LooCast
{
    public class TypeManager : Manager
    {
        #region Static Properties
        public static TypeManager Instance
        {
            get
            {
                if (instance == null)
                {
                    GameObject instanceObject = new GameObject("[TypeManager]");
                    instanceObject.layer = 31;
                    instanceObject.tag = "INTERNAL";
                    return instanceObject.AddComponent<TypeManager>();
                }
                else
                {
                    return instance;
                }
            }
        }
        #endregion

        #region Static Fields
        private static TypeManager instance;
        #endregion

        #region Methods
        public void RegisterType(Type type)
        {
            Registry<IIdentifier, IIdentifiable> typeRegistry = RegistryManager.Instance.GetRegistry("LooCast:TypeIdentifier_LooCast:Type");
            typeRegistry.Register(type.TypeIdentifier, type);
        }

        public Type GetType(TypeIdentifier typeIdentifier)
        {
            Registry<IIdentifier, IIdentifiable> typeRegistry = RegistryManager.Instance.GetRegistry("LooCast:TypeIdentifier_LooCast:Type");
            return (Type)typeRegistry.Get(typeIdentifier);
        }
        #endregion
    }
}