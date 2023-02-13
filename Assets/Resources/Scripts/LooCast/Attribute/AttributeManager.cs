using System;
using UnityEngine;

namespace LooCast.Attribute
{
    using Core;
    using Identifier;
    using Stat;
    
    public class AttributeManager : ModuleManager
    {
        #region Static Properties
        public static AttributeManager Instance
        {
            get
            {
                if (instance == null)
                {
                    GameObject instanceObject = new GameObject("[AttributeManager]");
                    instanceObject.layer = 31;
                    instanceObject.tag = "INTERNAL";
                    return instanceObject.AddComponent<AttributeManager>();
                }
                else
                {
                    return instance;
                }
            }
        }
        #endregion

        #region Static Fields
        private static AttributeManager instance;
        #endregion

        #region Properties
        public override SubModuleManager[] SubModuleManagers => subModuleManagers;
        #endregion

        #region Fields
        private SubModuleManager[] subModuleManagers;
        #endregion

        #region Methods
        public override void PreInitialize()
        {
            subModuleManagers = new SubModuleManager[]
            {
                StatManager.Instance
            };
        }

        public override void Initialize()
        {

        }

        public override void PostInitialize()
        {

        }
        #endregion
    }
}