using System;
using UnityEngine;

namespace LooCast.Data
{
    using Runtime;
    
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
                    GameObject instanceObject = new GameObject("[DataManager]");
                    instanceObject.layer = 31;
                    instanceObject.tag = "INTERNAL";
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
            InstanceManager instanceManager = InstanceManager.Instance;

            Namespace rootNamespace = namespaceManager.GetNamespace("LooCast");
            looCastNamespace = new Namespace("Data", rootNamespace);
            Namespace runtimeNamespace = new Namespace("Runtime", looCastNamespace);
            looCastType = new Type(typeof(DataManager), looCastNamespace);
            looCastInstance = new Instance(this, looCastType);

            namespaceManager.RegisterNamespace(looCastNamespace);
            namespaceManager.RegisterNamespace(runtimeNamespace);
            typeManager.RegisterType(looCastType);
            instanceManager.RegisterInstance(looCastInstance);
            
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