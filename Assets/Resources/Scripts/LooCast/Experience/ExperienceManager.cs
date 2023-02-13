using System;
using UnityEngine;

namespace LooCast.Core
{
    using Core.Manager;
    using Identifier;
    
    public class ExperienceManager : ModuleManager
    {
        #region Static Properties
        public static ExperienceManager Instance
        {
            get
            {
                if (instance == null)
                {
                    GameObject instanceObject = new GameObject("[ExperienceManager]");
                    instanceObject.layer = 31;
                    instanceObject.tag = "INTERNAL";
                    return instanceObject.AddComponent<ExperienceManager>();
                }
                else
                {
                    return instance;
                }
            }
        }
        #endregion

        #region Static Fields
        private static ExperienceManager instance;
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