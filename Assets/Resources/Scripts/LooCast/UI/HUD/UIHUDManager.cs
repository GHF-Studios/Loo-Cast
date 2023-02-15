﻿using System;
using UnityEngine;

namespace LooCast.UI.HUD
{
    using LooCast.System;
    using LooCast.System.Management;

    public class UIHUDManager : SubModuleManager
    {
        #region Static Properties
        public static UIHUDManager Instance
        {
            get
            {
                if (instance == null)
                {
                    GameObject instanceObject = new GameObject("[UIHUDManager]");
                    instanceObject.layer = 31;
                    instanceObject.tag = "INTERNAL";
                    DontDestroyOnLoad(instanceObject);
                    instanceObject.transform.parent = UIManager.Instance.transform;
                    return instanceObject.AddComponent<UIHUDManager>();
                }
                else
                {
                    return instance;
                }
            }
        }
        #endregion

        #region Static Fields
        private static UIHUDManager instance;
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
            looCastNamespace = new Namespace("HUD", rootNamespace);
            looCastType = new Type(typeof(UIHUDManager), looCastNamespace);
            looCastInstance = new Instance(this, looCastType);

            namespaceManager.RegisterNamespace(looCastNamespace);
            typeManager.RegisterType(looCastType);
            instanceManager.RegisterInstance(looCastInstance);

            Type hudType = new Type(typeof(HUD), looCastNamespace);

            typeManager.RegisterType(hudType);
            #endregion
        }
        #endregion
    }
}