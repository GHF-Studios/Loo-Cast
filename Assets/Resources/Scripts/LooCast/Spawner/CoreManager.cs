using System;
using UnityEngine;

namespace LooCast.Core
{
    using Core.Manager;
    using Identifier;
    
    public class DataManager : ModuleManager
    {
        #region Static Properties
        public static DataManager Instance
        {
            get
            {
                if (instance == null)
                {
                    GameObject instanceObject = new GameObject("[CoreManager]");
                    instanceObject.layer = 31;
                    instanceObject.tag = "INTERNAL";
                    return instanceObject.AddComponent<DataManager>();
                }
                else
                {
                    return instance;
                }
            }
        }
        #endregion

        #region Static Fields
        private static DataManager instance;
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