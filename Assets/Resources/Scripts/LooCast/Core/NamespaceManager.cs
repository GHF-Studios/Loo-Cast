using System;
using UnityEngine;

namespace LooCast.Core
{
    using Identifier;
    
    public class NamespaceManager : MonoBehaviour
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

        #region Fields
        #endregion

        #region Methods
        internal void Initialize()
        {
            
        }
        #endregion
    }
}