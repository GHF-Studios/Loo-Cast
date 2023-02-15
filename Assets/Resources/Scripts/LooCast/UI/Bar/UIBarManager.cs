using System;
using UnityEngine;

namespace LooCast.UI.Bar
{
    using LooCast.System;
    using LooCast.System.Management;

    public class UIBarManager : SubModuleManager
    {
        #region Static Properties
        public static UIBarManager Instance
        {
            get
            {
                if (instance == null)
                {
                    GameObject instanceObject = new GameObject("[UIBarManager]");
                    instanceObject.layer = 31;
                    instanceObject.tag = "INTERNAL";
                    DontDestroyOnLoad(instanceObject);
                    instanceObject.transform.parent = UIManager.Instance.transform;
                    return instanceObject.AddComponent<UIBarManager>();
                }
                else
                {
                    return instance;
                }
            }
        }
        #endregion

        #region Static Fields
        private static UIBarManager instance;
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
            looCastNamespace = new Namespace("Bar", rootNamespace);
            looCastType = new Type(typeof(UIBarManager), looCastNamespace);
            looCastInstance = new Instance(this, looCastType);

            namespaceManager.RegisterNamespace(looCastNamespace);
            typeManager.RegisterType(looCastType);
            instanceManager.RegisterInstance(looCastInstance);

            Type barType = new Type(typeof(Bar), looCastNamespace);
            Type energyBarType = new Type(typeof(EnergyBar), looCastNamespace);
            Type experienceBarType = new Type(typeof(ExperienceBar), looCastNamespace);
            Type healthBarType = new Type(typeof(HealthBar), looCastNamespace);

            typeManager.RegisterType(barType);
            typeManager.RegisterType(energyBarType);
            typeManager.RegisterType(experienceBarType);
            typeManager.RegisterType(healthBarType);
            #endregion
        }
        #endregion
    }
}