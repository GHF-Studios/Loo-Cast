using System;
using UnityEngine;

namespace LooCast.Arc
{
    using Core.Manager;
    using Identifier;
    
    public class ArcManager : ModuleManager
    {
        #region Static Properties
        public static ArcManager Instance
        {
            get
            {
                if (instance == null)
                {
                    GameObject instanceObject = new GameObject("[ArcManager]");
                    instanceObject.layer = 31;
                    instanceObject.tag = "INTERNAL";
                    return instanceObject.AddComponent<ArcManager>();
                }
                else
                {
                    return instance;
                }
            }
        }
        #endregion

        #region Static Fields
        private static ArcManager instance;
        #endregion

        #region Fields

        #endregion

        #region Methods
        public override void PreInitialize()
        {

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