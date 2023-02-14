using System;
using UnityEngine;

namespace LooCast
{
    public class NamespaceManager : Manager
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

        #region Methods
        public void RegisterNamespace(Namespace @namespace)
        {
            // TODO: Implement
        }

        public Namespace GetNamespace(NamespaceIdentifier namespaceIdentifier)
        {
            // TODO: Implement
        }
        #endregion
    }
}