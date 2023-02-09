using System;
using UnityEngine;

namespace LooCast.Core
{
    using Identifier;

    public class TypeManager : MonoBehaviour
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

        #region Fields
        #endregion

        #region Methods
        internal void Initialize()
        {
            
        }
        #endregion
    }
}