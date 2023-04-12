using System;
using UnityEngine;

namespace LooCast.System.Managers
{
    using global::LooCast.System.Identifiers;
    using global::LooCast.System.Registries;
    
    public sealed class NamespaceManager : InternalManager
    {
        #region Static Properties
        public static NamespaceManager Instance
        {
            get
            {
                if (instance == null)
                {
                    instance = new NamespaceManager();
                    instance.UnityEngineGameObject.name = "[NamespaceManager]";
                    instance.UnityEngineGameObject.layer = 31;
                    instance.UnityEngineGameObject.tag = "INTERNAL";
                    instance.UnityEngineGameObject.transform.parent = LooCast.Instance.gameObject.transform;
                }
                return instance;
            }
        }
        #endregion

        #region Static Fields
        private static NamespaceManager instance;
        #endregion

        #region Constructors
        public NamespaceManager() : base("LooCast.System.Managers.NamespaceManager", MainManager.Instance)
        {
            namespaceRegistry = new NamespaceRegistry();
        }
        #endregion

        #region Properties
        #endregion

        #region Fields
        private NamespaceRegistry namespaceRegistry;
        #endregion

        #region Methods
        public void RegisterNamespace(Namespace @namespace)
        {
            NamespaceIdentifier namespaceIdentifier = @namespace.NamespaceIdentifier;
            if (namespaceRegistry.ContainsKey(namespaceIdentifier))
            {
                throw new Exception($"[NamespaceManager] Namespace '{namespaceIdentifier}' already exists!");
            }
            namespaceRegistry.Add(@namespace.NamespaceIdentifier, @namespace);
        }

        public Namespace GetNamespace(NamespaceIdentifier namespaceIdentifier)
        {
            if (!namespaceRegistry.TryGetValue(namespaceIdentifier, out Namespace @namespace))
            {
                throw new Exception($"[NamespaceManager] Namespace '{namespaceIdentifier}' could not be found!");
            }
            return @namespace;
        }
        #endregion

        #region Overrides
        public override void PreInitializeInstance()
        {
            base.PreInitializeInstance();
        }

        public override void InitializeInstance()
        {
            base.InitializeInstance();
        }

        public override void PostInitializeInstance()
        {
            base.PostInitializeInstance();
        }
        #endregion
    }
}