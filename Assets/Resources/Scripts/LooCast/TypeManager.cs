using System;
using UnityEngine;

namespace LooCast
{
    public class TypeManager : InternalManager
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
                    instanceObject.transform.parent = Core.CoreManager.Instance.transform;
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

        #region Properties
        #endregion

        #region Fields
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

        #region Overrides
        public override void InitializeInstance()
        {
            base.InitializeInstance();

            #region Namespace/Type/Instance Registration
            NamespaceManager namespaceManager = NamespaceManager.Instance;
            InstanceManager instanceManager = InstanceManager.Instance;

            looCastNamespace = namespaceManager.GetNamespace("LooCast");
            looCastType = new Type(typeof(TypeManager), looCastNamespace);
            looCastInstance = new Instance(this, looCastType);

            RegisterType(looCastType);
            instanceManager.RegisterInstance(looCastInstance);
            #endregion
        }
        #endregion
    }
}