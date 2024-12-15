using System;
using UnityEngine;

namespace LooCast.Mission
{
    using LooCast.System;
    using LooCast.System.Managers;
    using LooCast.Mission.Reward;
    using LooCast.Mission.Target;
    using LooCast.Mission.Task;
    using LooCast.Mission.Trigger;
    
    public class MissionManager : ModuleManager
    {
        #region Static Properties
        public static MissionReceiver Instance
        {
            get
            {
                if (instance == null)
                {
                    UnityEngine.GameObject instanceObject = new UnityEngine.GameObject("[MissionManager]");
                    instanceObject.layer = 31;
                    instanceObject.tag = "INTERNAL";
                    DontDestroyOnLoad(instanceObject);
                    instanceObject.transform.parent = Core.CoreManager.Instance.transform;
                    return instanceObject.AddComponent<MissionReceiver>();
                }
                else
                {
                    return instance;
                }
            }
        }
        #endregion

        #region Static Fields
        private static MissionReceiver instance;
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
            UnityInstanceManager unityInstanceManager = UnityInstanceManager.Instance;

            INamespace rootNamespace = namespaceManager.GetNamespace("LooCast");
            looCastNamespace = new Namespace("Mission", rootNamespace);
            looCastType = new Type(typeof(MissionManager), looCastNamespace);
            looCastUnityInstance = new UnityInstance(this, (UnityInstanceType)looCastType);

            namespaceManager.RegisterNamespace(looCastNamespace);
            typeManager.RegisterType(looCastType);
            unityInstanceManager.RegisterUnityInstance(looCastUnityInstance);

            Type conquerStationMissionType = new Type(typeof(ConquerStationMission), looCastNamespace);
            Type missionType = new Type(typeof(Mission), looCastNamespace);
            Type missionProviderType = new Type(typeof(MissionProvider), looCastNamespace);
            Type missionRarityType = new Type(typeof(MissionRarity), looCastNamespace);
            Type missionReceiverType = new Type(typeof(MissionReceiver), looCastNamespace);
            Type missionStateType = new Type(typeof(MissionState), looCastNamespace);

            typeManager.RegisterType(missionType);
            typeManager.RegisterType(missionProviderType);
            typeManager.RegisterType(missionRarityType);
            typeManager.RegisterType(missionReceiverType);
            typeManager.RegisterType(missionStateType);
            typeManager.RegisterType(conquerStationMissionType);
            #endregion
        }

        protected override SubModuleManager[] GetSubModuleManagers()
        {
            return new SubModuleManager[]
            {
                MissionRewardManager.Instance,
                MissionTargetManager.Instance,
                MissionTaskManager.Instance,
                MissionTriggerManager.Instance
            };
        }
        #endregion
    }
}