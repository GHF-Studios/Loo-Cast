using System;
using UnityEngine;

namespace LooCast.Core.Instance
{
    using Core.Manager;
    using Identifier;

    public class InstanceManager : SubModuleManager
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

            };
        }

        public override void Initialize()
        {

        }

        public override void PostInitialize()
        {

        }

        public void RegisterInstance(Instance instance)
        {
            // TODO: Implement
        }

        public void UnregisterInstance(Instance instance)
        {
            // TODO: Implement
        }

        public Instance GetInstance(InstanceIdentifier instanceIdentifier)
        {
            // TODO: Implement
        }
        #endregion
    }
}