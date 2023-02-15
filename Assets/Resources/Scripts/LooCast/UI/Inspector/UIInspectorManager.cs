using System;
using UnityEngine;

namespace LooCast.UI.Inspector
{
    public class UIInspectorManager : SubModuleManager
    {
        #region Static Properties
        public static UIInspectorManager Instance
        {
            get
            {
                if (instance == null)
                {
                    GameObject instanceObject = new GameObject("[UIInspectorManager]");
                    instanceObject.layer = 31;
                    instanceObject.tag = "INTERNAL";
                    DontDestroyOnLoad(instanceObject);
                    instanceObject.transform.parent = UIManager.Instance.transform;
                    return instanceObject.AddComponent<UIInspectorManager>();
                }
                else
                {
                    return instance;
                }
            }
        }
        #endregion

        #region Static Fields
        private static UIInspectorManager instance;
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
            looCastNamespace = new Namespace("Inspector", rootNamespace);
            looCastType = new Type(typeof(UIInspectorManager), looCastNamespace);
            looCastInstance = new Instance(this, looCastType);

            namespaceManager.RegisterNamespace(looCastNamespace);
            typeManager.RegisterType(looCastType);
            instanceManager.RegisterInstance(looCastInstance);

            Type activeMissionInspectorType = new Type(typeof(ActiveMissionInspector), looCastNamespace);
            Type asteroidInspectorType = new Type(typeof(AsteroidInspector), looCastNamespace);

            typeManager.RegisterType(activeMissionInspectorType);
            typeManager.RegisterType(asteroidInspectorType);
            #endregion
        }
        #endregion
    }
}