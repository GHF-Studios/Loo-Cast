using System;
using UnityEngine;

namespace LooCast.Util
{
    using LooCast.Util.Collections.Concurrent;
    using LooCast.Util.Collections.Generic;
    using LooCast.System;
    using LooCast.System.Management;

    public class UtilManager : ModuleManager
    {
        #region Static Properties
        public static UtilManager Instance
        {
            get
            {
                if (instance == null)
                {
                    GameObject instanceObject = new GameObject("[UtilManager]");
                    instanceObject.layer = 31;
                    instanceObject.tag = "INTERNAL";
                    DontDestroyOnLoad(instanceObject);
                    instanceObject.transform.parent = Core.CoreManager.Instance.transform;
                    return instanceObject.AddComponent<UtilManager>();
                }
                else
                {
                    return instance;
                }
            }
        }
        #endregion

        #region Static Fields
        private static UtilManager instance;
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

            Namespace rootNamespace = namespaceManager.GetNamespace("LooCast");
            looCastNamespace = new Namespace("Util", rootNamespace);
            Namespace collectionsNamespace = new Namespace("Collections", looCastNamespace);
            Namespace collectionsConcurrentNamespace = new Namespace("Concurrent", collectionsNamespace);
            Namespace collectionsGenericNamespace = new Namespace("Generic", collectionsNamespace);
            looCastType = new Type(typeof(UtilManager), looCastNamespace);
            looCastUnityInstance = new Instance(this, looCastType);

            namespaceManager.RegisterNamespace(looCastNamespace);
            namespaceManager.RegisterNamespace(collectionsNamespace);
            namespaceManager.RegisterNamespace(collectionsConcurrentNamespace);
            namespaceManager.RegisterNamespace(collectionsGenericNamespace);
            typeManager.RegisterType(looCastType);
            instanceManager.RegisterInstance(looCastUnityInstance);

            Type colorUtilType = new Type(typeof(ColorUtil), looCastNamespace);
            Type constantsType = new Type(typeof(Constants), looCastNamespace);
            Type extensionMethodsType = new Type(typeof(ExtensionMethods), looCastNamespace);
            Type lerpFollowerType = new Type(typeof(LerpFollower), looCastNamespace);
            Type rectTransformUtilType = new Type(typeof(RectTransformUtil), looCastNamespace);
            Type screenShakeType = new Type(typeof(ScreenShake), looCastNamespace);
            Type serializationUtilType = new Type(typeof(SerializationUtil), looCastNamespace);
            Type targetingUtilType = new Type(typeof(TargetingUtil), looCastNamespace);
            Type teamUtilType = new Type(typeof(TeamUtil), looCastNamespace);
            Type textureUtilType = new Type(typeof(TextureUtil), looCastNamespace);
            Type timerUtilType = new Type(typeof(TimerUtil), looCastNamespace);
            Type concurrentSerializableDictionaryType = new Type(typeof(ConcurrentSerializableDictionary<object, object>), collectionsConcurrentNamespace);
            Type serializableDictionaryType = new Type(typeof(SerializableDictionary<object, object>), collectionsGenericNamespace);
            Type serializableListType = new Type(typeof(SerializableList<object>), collectionsGenericNamespace);

            typeManager.RegisterType(colorUtilType);
            typeManager.RegisterType(constantsType);
            typeManager.RegisterType(extensionMethodsType);
            typeManager.RegisterType(lerpFollowerType);
            typeManager.RegisterType(rectTransformUtilType);
            typeManager.RegisterType(screenShakeType);
            typeManager.RegisterType(serializationUtilType);
            typeManager.RegisterType(targetingUtilType);
            typeManager.RegisterType(teamUtilType);
            typeManager.RegisterType(textureUtilType);
            typeManager.RegisterType(timerUtilType);
            typeManager.RegisterType(concurrentSerializableDictionaryType);
            typeManager.RegisterType(serializableDictionaryType);
            typeManager.RegisterType(serializableListType);
            #endregion
        }
        #endregion
    }
}