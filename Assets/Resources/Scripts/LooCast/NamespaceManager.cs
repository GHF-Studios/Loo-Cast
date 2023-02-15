using System;
using UnityEngine;

namespace LooCast
{
    public class NamespaceManager : InternalManager
    {
        #region Static Properties
        public static NamespaceManager Instance
        {
            get
            {
                if (instance == null)
                {
                    GameObject instanceObject = new GameObject("[NamespaceManager]");
                    instanceObject.layer = 31;
                    instanceObject.tag = "INTERNAL";
                    instanceObject.transform.parent = Core.CoreManager.Instance.transform;
                    return instanceObject.AddComponent<NamespaceManager>();
                }
                else
                {
                    return instance;
                }
            }
        }
        #endregion

        #region Static Fields
        private static NamespaceManager instance;
        #endregion

        #region Properties
        #endregion

        #region Fields
        #endregion

        #region Methods
        public void RegisterNamespace(Namespace @namespace)
        {
            Registry<IIdentifier, IIdentifiable> namespaceRegistry = RegistryManager.Instance.GetRegistry("LooCast:NamespaceIdentifier_LooCast:Namespace");
            namespaceRegistry.Register(@namespace.NamespaceIdentifier, @namespace);
        }

        public Namespace GetNamespace(NamespaceIdentifier namespaceIdentifier)
        {
            Registry<IIdentifier, IIdentifiable> namespaceRegistry = RegistryManager.Instance.GetRegistry("LooCast:NamespaceIdentifier_LooCast:Namespace");
            return (Namespace)namespaceRegistry.Get(namespaceIdentifier);
        }
        #endregion

        #region Overrides
        public override void InitializeInstance()
        {
            base.InitializeInstance();

            #region Namespace/Type/Instance Registration
            TypeManager typeManager = TypeManager.Instance;
            InstanceManager instanceManager = InstanceManager.Instance;

            looCastNamespace = new Namespace("LooCast");
            looCastType = new Type(typeof(NamespaceManager), looCastNamespace);
            looCastInstance = new Instance(this, looCastType);

            RegisterNamespace(looCastNamespace);
            typeManager.RegisterType(looCastType);
            instanceManager.RegisterInstance(looCastInstance);
            #endregion
        }
        #endregion
    }
}