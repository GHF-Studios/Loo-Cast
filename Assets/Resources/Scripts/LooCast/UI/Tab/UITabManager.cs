using System;
using UnityEngine;

namespace LooCast.UI.Tab
{
    public class UITabManager : SubModuleManager
    {
        #region Static Properties
        public static UITabManager Instance
        {
            get
            {
                if (instance == null)
                {
                    GameObject instanceObject = new GameObject("[UITabManager]");
                    instanceObject.layer = 31;
                    instanceObject.tag = "INTERNAL";
                    DontDestroyOnLoad(instanceObject);
                    instanceObject.transform.parent = UIManager.Instance.transform;
                    return instanceObject.AddComponent<UITabManager>();
                }
                else
                {
                    return instance;
                }
            }
        }
        #endregion

        #region Static Fields
        private static UITabManager instance;
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
            looCastNamespace = new Namespace("Tab", rootNamespace);
            looCastType = new Type(typeof(UITabManager), looCastNamespace);
            looCastInstance = new Instance(this, looCastType);

            namespaceManager.RegisterNamespace(looCastNamespace);
            typeManager.RegisterType(looCastType);
            instanceManager.RegisterInstance(looCastInstance);

            Type tabType = new Type(typeof(Tab), looCastNamespace);
            Type tabGroupType = new Type(typeof(TabGroup), looCastNamespace);

            typeManager.RegisterType(tabType);
            typeManager.RegisterType(tabGroupType);
            #endregion
        }
        #endregion
    }
}