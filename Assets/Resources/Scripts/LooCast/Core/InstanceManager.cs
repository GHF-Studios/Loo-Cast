using System;
using UnityEngine;

namespace LooCast.Core
{
    using Identifier;

    public class InstanceManager : MonoBehaviour
    {
        #region Static Properties
        public static InstanceManager Instance
        {
            get
            {
                if (instance == null)
                {
                    GameObject instanceObject = new GameObject("[InstanceManager]");
                    instanceObject.layer = 31;
                    instanceObject.tag = "INTERNAL";
                    return instanceObject.AddComponent<InstanceManager>();
                }
                else
                {
                    return instance;
                }
            }
        }
        #endregion

        #region Static Fields
        private static InstanceManager instance;
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