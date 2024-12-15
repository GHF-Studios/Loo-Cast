﻿using System;
using UnityEngine;

namespace LooCast.UI.Title
{
    using LooCast.System;
    using LooCast.System.Managers;

    public class UITitleManager : SubModuleManager
    {
        #region Static Properties
        public static UITitleManager Instance
        {
            get
            {
                if (instance == null)
                {
                    UnityEngine.GameObject instanceObject = new UnityEngine.GameObject("[UITitleManager]");
                    instanceObject.layer = 31;
                    instanceObject.tag = "INTERNAL";
                    DontDestroyOnLoad(instanceObject);
                    instanceObject.transform.parent = UIManager.Instance.transform;
                    return instanceObject.AddComponent<UITitleManager>();
                }
                else
                {
                    return instance;
                }
            }
        }
        #endregion

        #region Static Fields
        private static UITitleManager instance;
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

            Namespace rootNamespace = namespaceManager.GetNamespace("LooCast.UI");
            looCastNamespace = new Namespace("Title", rootNamespace);
            looCastType = new Type(typeof(UITitleManager), looCastNamespace);
            looCastUnityInstance = new UnityInstance(this, (UnityInstanceType)looCastType);

            namespaceManager.RegisterNamespace(looCastNamespace);
            typeManager.RegisterType(looCastType);
            unityInstanceManager.RegisterUnityInstance(looCastUnityInstance);

            Type titleType = new Type(typeof(Title), looCastNamespace);

            typeManager.RegisterType(titleType);
            #endregion
        }
        #endregion
    }
}