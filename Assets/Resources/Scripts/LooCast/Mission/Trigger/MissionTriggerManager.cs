using System;
using UnityEngine;

namespace LooCast.Mission.Trigger
{
    public class MissionTriggerManager : SubModuleManager
    {
        #region Static Properties
        public static MissionTriggerManager Instance
        {
            get
            {
                if (instance == null)
                {
                    GameObject instanceObject = new GameObject("[MissionTriggerManager]");
                    instanceObject.layer = 31;
                    instanceObject.tag = "INTERNAL";
                    DontDestroyOnLoad(instanceObject);
                    instanceObject.transform.parent = MissionManager.Instance.transform;
                    return instanceObject.AddComponent<MissionTriggerManager>();
                }
                else
                {
                    return instance;
                }
            }
        }
        #endregion

        #region Static Fields
        private static MissionTriggerManager instance;
        #endregion

        #region Fields

        #endregion

        #region Methods
        #endregion

        #region Overrides
        public override void PreInitializeInstance()
        {
            base.PreInitializeInstance();

            #region Namespace/Type/Instance Registration
            NamespaceManager namespaceManager = NamespaceManager.Instance;
            TypeManager typeManager = TypeManager.Instance;
            InstanceManager instanceManager = InstanceManager.Instance;

            Namespace rootNamespace = namespaceManager.GetNamespace("LooCast.Mission");
            looCastNamespace = new Namespace("Trigger", rootNamespace);
            looCastType = new Type(typeof(MissionTriggerManager), looCastNamespace);
            looCastInstance = new Instance(this, looCastType);

            namespaceManager.RegisterNamespace(looCastNamespace);
            typeManager.RegisterType(looCastType);
            instanceManager.RegisterInstance(looCastInstance);

            Type missionTriggerType = new Type(typeof(MissionTrigger), looCastNamespace);

            typeManager.RegisterType(missionTriggerType);
            #endregion
        }
        #endregion
    }
}