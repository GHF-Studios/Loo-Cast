using System;
using UnityEngine;

namespace LooCast.Core.Instance
{
    using Identifier;
    using LooCast.Core.Registry;

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
        public override Manager[] SubManagers => subManagers;
        #endregion

        #region Fields
        private Manager[] subManagers;

        #endregion

        #region Methods
        public override void PreInitialize()
        {
            subManagers = new Manager[]
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

        }

        public void UnregisterInstance(Instance instance)
        {

        }

        public Instance GetInstance(InstanceIdentifier instanceIdentifier)
        {
            
        }
        #endregion
    }
}