using System;
using UnityEngine;

namespace LooCast.Mission.Trigger
{
    using LooCast.System;
    using LooCast.System.Managers;

    public class MissionTriggerManager : SubModuleManager
    {
        #region Static Properties
        public static MissionTriggerManager Instance
        {
            get
            {
                if (instance == null)
                {
                    UnityEngine.GameObject instanceObject = new UnityEngine.GameObject("[MissionTriggerManager]");
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
            UnityInstanceManager unityInstanceManager = UnityInstanceManager.Instance;

            Namespace rootNamespace = namespaceManager.GetNamespace("LooCast.Mission");
            looCastNamespace = new Namespace("Trigger", rootNamespace);
            looCastType = new Type(typeof(MissionTriggerManager), looCastNamespace);
            looCastUnityInstance = new UnityInstance(this, (UnityInstanceType)looCastType);

            namespaceManager.RegisterNamespace(looCastNamespace);
            typeManager.RegisterType(looCastType);
            unityInstanceManager.RegisterUnityInstance(looCastUnityInstance);

            Type missionTriggerType = new Type(typeof(MissionTrigger), looCastNamespace);

            typeManager.RegisterType(missionTriggerType);
            #endregion
        }
        #endregion
    }
}