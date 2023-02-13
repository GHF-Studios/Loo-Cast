using System;
using UnityEngine;

namespace LooCast.Core
{
    using Core.Manager;
    using Identifier;
    
    public class EventManager : ModuleManager
    {
        #region Static Properties
        public static EventManager Instance
        {
            get
            {
                if (instance == null)
                {
                    GameObject instanceObject = new GameObject("[EventManager]");
                    instanceObject.layer = 31;
                    instanceObject.tag = "INTERNAL";
                    return instanceObject.AddComponent<EventManager>();
                }
                else
                {
                    return instance;
                }
            }
        }
        #endregion

        #region Static Fields
        private static EventManager instance;
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