﻿using System;
using UnityEngine;

namespace LooCast.Data
{
    using LooCast.System;
    using LooCast.System.Management;
    using LooCast.Data.Runtime;
    
    // TODO: Implement Data System and Internal Data System
    public class DataManager : ModuleManager
    {
        #region Static Properties
        public static DataManager Instance
        {
            get
            {
                if (instance == null)
                {
                    UnityEngine.GameObject instanceObject = new UnityEngine.GameObject("[DataManager]");
                    instanceObject.layer = 31;
                    instanceObject.tag = "INTERNAL";
                    DontDestroyOnLoad(instanceObject);
                    instanceObject.transform.parent = Core.CoreManager.Instance.transform;
                    return instanceObject.AddComponent<DataManager>();
                }
                else
                {
                    return instance;
                }
            }
        }
        #endregion

        #region Static Fields
        private static DataManager instance;
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
            looCastNamespace = new Namespace("Data", rootNamespace);
            Namespace runtimeNamespace = new Namespace("Runtime", looCastNamespace);
            looCastType = new Type(typeof(DataManager), looCastNamespace);
            looCastUnityInstance = new UnityInstance(this, (UnityInstanceType)looCastType);

            namespaceManager.RegisterNamespace(looCastNamespace);
            namespaceManager.RegisterNamespace(runtimeNamespace);
            typeManager.RegisterType(looCastType);
            unityInstanceManager.RegisterUnityInstance(looCastUnityInstance);
            
            Type boolDataType = new Type(typeof(BoolData), looCastNamespace);
            Type boolDataReferenceType = new Type(typeof(BoolDataReference), looCastNamespace);
            Type dataType = new Type(typeof(Data), looCastNamespace);
            Type dynamicGameDataType = new Type(typeof(DynamicGameData), looCastNamespace);
            Type floatDataType = new Type(typeof(FloatData), looCastNamespace);
            Type floatDataReferenceType = new Type(typeof(FloatDataReference), looCastNamespace);
            Type intDataType = new Type(typeof(IntData), looCastNamespace);
            Type intDataReferenceType = new Type(typeof(IntDataReference), looCastNamespace);
            Type stringDataType = new Type(typeof(StringData), looCastNamespace);
            Type stringDataReferenceType = new Type(typeof(StringDataReference), looCastNamespace);
            Type iRuntimeDataDeserializerType = new Type(typeof(IRuntimeDataDeserializer), runtimeNamespace);
            Type iRuntimeDataSerializerType = new Type(typeof(IRuntimeDataSerializer), runtimeNamespace);
            Type runtimeDataType = new Type(typeof(RuntimeData), runtimeNamespace);
            Type runtimeSetType = new Type(typeof(RuntimeSet<object>), runtimeNamespace);
            Type runtimeSetsType = new Type(typeof(RuntimeSets), runtimeNamespace);

            typeManager.RegisterType(boolDataType);
            typeManager.RegisterType(boolDataReferenceType);
            typeManager.RegisterType(dataType);
            typeManager.RegisterType(dynamicGameDataType);
            typeManager.RegisterType(floatDataType);
            typeManager.RegisterType(floatDataReferenceType);
            typeManager.RegisterType(intDataType);
            typeManager.RegisterType(intDataReferenceType);
            typeManager.RegisterType(stringDataType);
            typeManager.RegisterType(stringDataReferenceType);
            typeManager.RegisterType(iRuntimeDataDeserializerType);
            typeManager.RegisterType(iRuntimeDataSerializerType);
            typeManager.RegisterType(runtimeDataType);
            typeManager.RegisterType(runtimeSetType);
            typeManager.RegisterType(runtimeSetsType);
            #endregion
        }
        #endregion
    }
}