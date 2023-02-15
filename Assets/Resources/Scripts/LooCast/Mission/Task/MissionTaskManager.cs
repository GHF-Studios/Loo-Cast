using System;
using UnityEngine;

namespace LooCast.Mission.Task
{
    public class MissionTaskManager : SubModuleManager
    {
        #region Static Properties
        public static MissionTaskManager Instance
        {
            get
            {
                if (instance == null)
                {
                    GameObject instanceObject = new GameObject("[MissionTaskManager]");
                    instanceObject.layer = 31;
                    instanceObject.tag = "INTERNAL";
                    DontDestroyOnLoad(instanceObject);
                    instanceObject.transform.parent = MissionManager.Instance.transform;
                    return instanceObject.AddComponent<MissionTaskManager>();
                }
                else
                {
                    return instance;
                }
            }
        }
        #endregion

        #region Static Fields
        private static MissionTaskManager instance;
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
            looCastNamespace = new Namespace("Task", rootNamespace);
            looCastType = new Type(typeof(MissionTaskManager), looCastNamespace);
            looCastInstance = new Instance(this, looCastType);

            namespaceManager.RegisterNamespace(looCastNamespace);
            typeManager.RegisterType(looCastType);
            instanceManager.RegisterInstance(looCastInstance);

            Type iMissionTaskLockStateType = new Type(typeof(IMissionTaskLockState), looCastNamespace);
            Type lockedMissionTaskLockStateType = new Type(typeof(LockedMissionTaskLockState), looCastNamespace);
            Type missionTaskType = new Type(typeof(MissionTask), looCastNamespace);
            Type unlockedMissionTaskLockStateType = new Type(typeof(UnlockedMissionTaskLockState), looCastNamespace);

            typeManager.RegisterType(iMissionTaskLockStateType);
            typeManager.RegisterType(lockedMissionTaskLockStateType);
            typeManager.RegisterType(missionTaskType);
            typeManager.RegisterType(unlockedMissionTaskLockStateType);
            #endregion
        }
        #endregion
    }
}