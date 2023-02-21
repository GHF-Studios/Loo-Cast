using System;
using UnityEngine;

namespace LooCast.UI.Task
{
    using LooCast.System;
    using LooCast.System.Management;

    public class UITaskManager : SubModuleManager
    {
        #region Static Properties
        public static UITaskManager Instance
        {
            get
            {
                if (instance == null)
                {
                    GameObject instanceObject = new GameObject("[UITaskManager]");
                    instanceObject.layer = 31;
                    instanceObject.tag = "INTERNAL";
                    DontDestroyOnLoad(instanceObject);
                    instanceObject.transform.parent = UIManager.Instance.transform;
                    return instanceObject.AddComponent<UITaskManager>();
                }
                else
                {
                    return instance;
                }
            }
        }
        #endregion

        #region Static Fields
        private static UITaskManager instance;
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

            Namespace rootNamespace = namespaceManager.GetNamespace("LooCast.UI");
            looCastNamespace = new Namespace("Task", rootNamespace);
            looCastType = new Type(typeof(UITaskManager), looCastNamespace);
            looCastUnityInstance = new Instance(this, looCastType);

            namespaceManager.RegisterNamespace(looCastNamespace);
            typeManager.RegisterType(looCastType);
            instanceManager.RegisterInstance(looCastUnityInstance);

            Type missionTaskType = new Type(typeof(MissionTask), looCastNamespace);
            Type missionTaskContainerType = new Type(typeof(MissionTaskContainer), looCastNamespace);

            typeManager.RegisterType(missionTaskType);
            typeManager.RegisterType(missionTaskContainerType);
            #endregion
        }
        #endregion
    }
}