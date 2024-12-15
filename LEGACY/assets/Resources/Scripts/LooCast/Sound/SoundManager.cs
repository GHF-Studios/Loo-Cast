﻿using System;
using UnityEngine;

namespace LooCast.Sound
{
    using LooCast.System;
    using LooCast.System.Managers;

    public class SoundManager : ModuleManager
    {
        #region Static Properties
        public static SoundManager Instance
        {
            get
            {
                if (instance == null)
                {
                    UnityEngine.GameObject instanceObject = new UnityEngine.GameObject("[SoundManager]");
                    instanceObject.layer = 31;
                    instanceObject.tag = "INTERNAL";
                    DontDestroyOnLoad(instanceObject);
                    instanceObject.transform.parent = Core.CoreManager.Instance.transform;
                    return instanceObject.AddComponent<SoundManager>();
                }
                else
                {
                    return instance;
                }
            }
        }
        #endregion

        #region Static Fields
        private static SoundManager instance;
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
            looCastNamespace = new Namespace("Sound", rootNamespace);
            looCastType = new Type(typeof(SoundManager), looCastNamespace);
            looCastUnityInstance = new UnityInstance(this, (UnityInstanceType)looCastType);

            namespaceManager.RegisterNamespace(looCastNamespace);
            typeManager.RegisterType(looCastType);
            unityInstanceManager.RegisterUnityInstance(looCastUnityInstance);

            Type gameSoundHandlerType = new Type(typeof(GameSoundHandler), looCastNamespace);
            Type menuSoundHandlerType = new Type(typeof(MenuSoundHandler), looCastNamespace);
            Type soundType = new Type(typeof(Sound), looCastNamespace);
            Type soundHandlerType = new Type(typeof(SoundHandler), looCastNamespace);

            typeManager.RegisterType(gameSoundHandlerType);
            typeManager.RegisterType(menuSoundHandlerType);
            typeManager.RegisterType(soundType);
            typeManager.RegisterType(soundHandlerType);
            #endregion
        }
        #endregion
    }
}