using System;
using UnityEngine;

namespace LooCast
{
    public class InstanceManager : Manager
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

        #region Methods
        public override void InitializeInstance()
        {
            base.InitializeInstance();
        }

        public void RegisterInstance(Instance instance)
        {
            Registry<IIdentifier, IIdentifiable> instanceRegistry = RegistryManager.Instance.GetRegistry("LooCast:InstanceIdentifier_LooCast:Instance");
            instanceRegistry.Register(instance.InstanceIdentifier, instance);
        }

        public void UnregisterInstance(Instance instance)
        {
            Registry<IIdentifier, IIdentifiable> instanceRegistry = RegistryManager.Instance.GetRegistry("LooCast:InstanceIdentifier_LooCast:Instance");
            instanceRegistry.Unregister(instance.InstanceIdentifier);
        }

        public Instance GetInstance(InstanceIdentifier instanceIdentifier)
        {
            Registry<IIdentifier, IIdentifiable> instanceRegistry = RegistryManager.Instance.GetRegistry("LooCast:InstanceIdentifier_LooCast:Instance");
            return (Instance)instanceRegistry.Get(instanceIdentifier);
        }
        #endregion
        }
}