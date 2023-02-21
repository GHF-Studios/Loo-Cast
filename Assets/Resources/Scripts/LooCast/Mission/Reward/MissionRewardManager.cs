using System;
using UnityEngine;

namespace LooCast.Mission.Reward
{
    using LooCast.System;
    using LooCast.System.Management;

    public class MissionRewardManager : SubModuleManager
    {
        #region Static Properties
        public static MissionRewardManager Instance
        {
            get
            {
                if (instance == null)
                {
                    UnityEngine.GameObject instanceObject = new UnityEngine.GameObject("[MissionRewardManager]");
                    instanceObject.layer = 31;
                    instanceObject.tag = "INTERNAL";
                    DontDestroyOnLoad(instanceObject);
                    instanceObject.transform.parent = MissionManager.Instance.transform;
                    return instanceObject.AddComponent<MissionRewardManager>();
                }
                else
                {
                    return instance;
                }
            }
        }
        #endregion

        #region Static Fields
        private static MissionRewardManager instance;
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

            Namespace rootNamespace = namespaceManager.GetNamespace("LooCast.Mission");
            looCastNamespace = new Namespace("Reward", rootNamespace);
            looCastType = new Type(typeof(MissionRewardManager), looCastNamespace);
            looCastUnityInstance = new UnityInstance(this, (UnityInstanceType)looCastType);

            namespaceManager.RegisterNamespace(looCastNamespace);
            typeManager.RegisterType(looCastType);
            unityInstanceManager.RegisterUnityInstance(looCastUnityInstance);

            Type creditsMissionRewardType = new Type(typeof(CreditsMissionReward), looCastNamespace);
            Type itemMissionRewardType = new Type(typeof(ItemMissionReward), looCastNamespace);
            Type missionRewardType = new Type(typeof(MissionReward), looCastNamespace);
            Type reputationMissionRewardType = new Type(typeof(ReputationMissionReward), looCastNamespace);

            typeManager.RegisterType(creditsMissionRewardType);
            typeManager.RegisterType(itemMissionRewardType);
            typeManager.RegisterType(missionRewardType);
            typeManager.RegisterType(reputationMissionRewardType);
            #endregion
        }
        #endregion
    }
}