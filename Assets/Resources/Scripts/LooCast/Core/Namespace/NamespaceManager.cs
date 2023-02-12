using System;
using UnityEngine;

namespace LooCast.Core.Namespace
{
    using Core.Manager;
    using Identifier;
    
    public class NamespaceManager : SubModuleManager
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
        public override Manager[] SubManagers => subManagers;
        #endregion

        #region Fields
        private Manager[] subManagers;
        #endregion

        #region Methods
        public override void PreInitialize()
        {
            subManagers = new Manager[]
            {

            };
        }

        public override void Initialize()
        {
            
        }

        public override void PostInitialize()
        {
            
        }

        public void RegisterNamespace(Namespace @namespace)
        {

        }

        public Namespace GetNamespace(NamespaceIdentifier namespaceIdentifier)
        {
            
        }
        #endregion
    }
}