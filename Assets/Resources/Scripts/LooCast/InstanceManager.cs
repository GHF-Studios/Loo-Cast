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

        #region Properties
        public override Namespace LooCastNamespace => throw new NotImplementedException();
        public override Type LooCastType => throw new NotImplementedException();
        public override Instance LooCastInstance => throw new NotImplementedException();
        #endregion

        #region Fields

        #endregion

        #region Methods
        public override void InitializeInstance()
        {
            base.InitializeInstance();
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