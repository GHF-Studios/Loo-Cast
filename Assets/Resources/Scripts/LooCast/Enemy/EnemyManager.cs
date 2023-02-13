using System;
using UnityEngine;

namespace LooCast.Core
{
    using Core.Manager;
    using Identifier;
    
    public class EnemyManager : ModuleManager
    {
        #region Static Properties
        public static EnemyManager Instance
        {
            get
            {
                if (instance == null)
                {
                    GameObject instanceObject = new GameObject("[EnemyManager]");
                    instanceObject.layer = 31;
                    instanceObject.tag = "INTERNAL";
                    return instanceObject.AddComponent<EnemyManager>();
                }
                else
                {
                    return instance;
                }
            }
        }
        #endregion

        #region Static Fields
        private static EnemyManager instance;
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