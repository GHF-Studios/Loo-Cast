using System;
using System.Collections.Generic;
using System.IO;
using System.Xml.Linq;
using System.Reflection;

namespace LooCast.System.Serialization
{
    public sealed class TypeInfo
    {
        #region Delegates
        public delegate void SerializePrimitiveDelegate(string primitiveName, object primitive, out XAttribute serializedPrimitive);
        public delegate void DeserializePrimitiveDelegate(XAttribute serializedPrimitive, out object primitive);

        public delegate void SerializeObjectDelegate(string objectName, object _object, out XElement serializedObject);
        public delegate void DeserializeObjectDelegate(XElement serializedObject, out object _object);

        public delegate void SerializeFileDelegate(string fileName, string fileExtension, string parentFolderPath, object file, out FileInfo serializedFile);
        public delegate void DeserializeFileDelegate(FileInfo serializedFile, out object file);

        public delegate void SerializeFolderDelegate(string folderName, string parentFolderPath, object folder, out DirectoryInfo serializedFolder);
        public delegate void DeserializeFolderDelegate(DirectoryInfo serializedFolder, out object folder);
        #endregion
        
        #region Classes
        public sealed class PreAnalysisInfo
        {
            #region Properties
            public TypeInfo TypeInfo { get; private set; }

            public HashSet<TypeInfo> UtilizedSubTypes { get; private set; }
            public HashSet<TypeInfo> GenericTypeArguments { get; private set; }
            public bool IsGeneric { get; private set; }
            public PropertyInfo[] Properties { get; private set; }
            public FieldInfo[] Fields { get; private set; }
            #endregion

            #region Static Methods
            public static PreAnalysisInfo Create_Normal(TypeInfo typeInfo, HashSet<TypeInfo> utilizedSubTypes, PropertyInfo[] properties, FieldInfo[] fields)
            {
                PreAnalysisInfo preAnalysisInfo = new PreAnalysisInfo();
                
                preAnalysisInfo.TypeInfo = typeInfo;
                preAnalysisInfo.UtilizedSubTypes = utilizedSubTypes;
                preAnalysisInfo.GenericTypeArguments = null;
                preAnalysisInfo.IsGeneric = false;
                preAnalysisInfo.Properties = properties;
                preAnalysisInfo.Fields = fields;

                return preAnalysisInfo;
            }

            public static PreAnalysisInfo Create_Generic(TypeInfo typeInfo, HashSet<TypeInfo> genericTypeArguments)
            {
                PreAnalysisInfo preAnalysisInfo = new PreAnalysisInfo();

                preAnalysisInfo.TypeInfo = typeInfo;
                preAnalysisInfo.UtilizedSubTypes = null;
                preAnalysisInfo.GenericTypeArguments = genericTypeArguments;
                preAnalysisInfo.IsGeneric = true;
                preAnalysisInfo.Properties = null;
                preAnalysisInfo.Fields = null;

                return preAnalysisInfo;
            }
            #endregion
        }

        public sealed class AnalysisInfo
        {
            #region Properties
            public TypeInfo TypeInfo { get; private set; }

            public Serializability Serializability { get; private set; }
            #endregion

            #region Static Methods
            public static AnalysisInfo Create(TypeInfo typeInfo, Serializability serializability)
            {
                if (serializability == Serializability.None)
                {
                    throw new InvalidOperationException($"Can not create analysis info of a type that is already known to be unserializable!");
                }
                
                AnalysisInfo analysisInfo = new AnalysisInfo();

                analysisInfo.TypeInfo = typeInfo;
                analysisInfo.Serializability = serializability;

                return analysisInfo;
            }
            #endregion
        }

        public sealed class PostAnalysisInfo
        {
            #region Properties
            public TypeInfo TypeInfo { get; private set; }

            public HashSet<TypeInfo> PrimitiveTypeDependencies { get; private set; }
            public HashSet<TypeInfo> ObjectTypeDependencies { get; private set; }
            public HashSet<TypeInfo> FileTypeDependencies { get; private set; }
            public HashSet<TypeInfo> FolderTypeDependencies { get; private set; }
            #endregion

            #region Static Methods
            public static PostAnalysisInfo Create_Primitive(TypeInfo typeInfo)
            {
                PostAnalysisInfo postAnalysisInfo = new PostAnalysisInfo();

                postAnalysisInfo.TypeInfo = typeInfo;
                postAnalysisInfo.PrimitiveTypeDependencies = null;
                postAnalysisInfo.ObjectTypeDependencies = null;
                postAnalysisInfo.FileTypeDependencies = null;
                postAnalysisInfo.FolderTypeDependencies = null;

                return postAnalysisInfo;
            }
            
            public static PostAnalysisInfo Create_Object(TypeInfo typeInfo, HashSet<TypeInfo> primitiveTypeDependencies, HashSet<TypeInfo> objectTypeDependencies)
            {
                PostAnalysisInfo postAnalysisInfo = new PostAnalysisInfo();

                postAnalysisInfo.TypeInfo = typeInfo;
                postAnalysisInfo.PrimitiveTypeDependencies = primitiveTypeDependencies;
                postAnalysisInfo.ObjectTypeDependencies = objectTypeDependencies;
                postAnalysisInfo.FileTypeDependencies = null;
                postAnalysisInfo.FolderTypeDependencies = null;

                return postAnalysisInfo;
            }

            public static PostAnalysisInfo Create_File(TypeInfo typeInfo, HashSet<TypeInfo> objectTypeDependencies)
            {
                PostAnalysisInfo postAnalysisInfo = new PostAnalysisInfo();

                postAnalysisInfo.TypeInfo = typeInfo;
                postAnalysisInfo.PrimitiveTypeDependencies = null;
                postAnalysisInfo.ObjectTypeDependencies = objectTypeDependencies;
                postAnalysisInfo.FileTypeDependencies = null;
                postAnalysisInfo.FolderTypeDependencies = null;

                return postAnalysisInfo;
            }

            public static PostAnalysisInfo Create_Folder(TypeInfo typeInfo, HashSet<TypeInfo> fileTypeDependencies, HashSet<TypeInfo> folderTypeDependencies)
            {
                PostAnalysisInfo postAnalysisInfo = new PostAnalysisInfo();

                postAnalysisInfo.TypeInfo = typeInfo;
                postAnalysisInfo.PrimitiveTypeDependencies = null;
                postAnalysisInfo.ObjectTypeDependencies = null;
                postAnalysisInfo.FileTypeDependencies = fileTypeDependencies;
                postAnalysisInfo.FolderTypeDependencies = folderTypeDependencies;

                return postAnalysisInfo;
            }
            #endregion
        }

        public sealed class PreProcessingInfo
        {
            #region Properties
            public TypeInfo TypeInfo { get; private set; }

            public bool OverrideSerialization { get; private set; }
            public bool OverrideDeserialization { get; private set; }
            #endregion

            #region Static Methods
            public static PreProcessingInfo Create_Normal(TypeInfo typeInfo, bool overrideSerialization, bool overrideDeserialization)
            {
                PreProcessingInfo preProcessingInfo = new PreProcessingInfo();

                preProcessingInfo.TypeInfo = typeInfo;
                preProcessingInfo.OverrideSerialization = overrideSerialization;
                preProcessingInfo.OverrideDeserialization = overrideDeserialization;

                return preProcessingInfo;
            }

            public static PreProcessingInfo Create_Generic(TypeInfo typeInfo)
            {
                PreProcessingInfo preProcessingInfo = new PreProcessingInfo();

                preProcessingInfo.TypeInfo = typeInfo;
                preProcessingInfo.OverrideSerialization = true;
                preProcessingInfo.OverrideDeserialization = true;

                return preProcessingInfo;
            }
            #endregion
        }

        public sealed class ProcessingInfo
        {
            #region Properties
            public TypeInfo TypeInfo { get; private set; }

            public SerializePrimitiveDelegate SerializePrimitiveDelegate { get; private set; }
            public DeserializePrimitiveDelegate DeserializePrimitiveDelegate { get; private set; }
            public SerializeObjectDelegate SerializeObjectDelegate { get; private set; }
            public DeserializeObjectDelegate DeserializeObjectDelegate { get; private set; }
            public SerializeFileDelegate SerializeFileDelegate { get; private set; }
            public DeserializeFileDelegate DeserializeFileDelegate { get; private set; }
            public SerializeFolderDelegate SerializeFolderDelegate { get; private set; }
            public DeserializeFolderDelegate DeserializeFolderDelegate { get; private set; }
            #endregion

            #region Static Methods
            public static ProcessingInfo Create_Primitive(TypeInfo typeInfo, SerializePrimitiveDelegate serializePrimitiveDelegate, DeserializePrimitiveDelegate deserializePrimitiveDelegate)
            {
                ProcessingInfo processingInfo = new ProcessingInfo();

                processingInfo.TypeInfo = typeInfo;
                processingInfo.SerializePrimitiveDelegate = serializePrimitiveDelegate;
                processingInfo.DeserializePrimitiveDelegate = deserializePrimitiveDelegate;
                processingInfo.SerializeObjectDelegate = null;
                processingInfo.DeserializeObjectDelegate = null;
                processingInfo.SerializeFileDelegate = null;
                processingInfo.DeserializeFileDelegate = null;
                processingInfo.SerializeFolderDelegate = null;
                processingInfo.DeserializeFolderDelegate = null;

                return processingInfo;
            }

            public static ProcessingInfo Create_Object(TypeInfo typeInfo, SerializeObjectDelegate serializeObjectDelegate, DeserializeObjectDelegate deserializeObjectDelegate)
            {
                ProcessingInfo processingInfo = new ProcessingInfo();

                processingInfo.TypeInfo = typeInfo;
                processingInfo.SerializePrimitiveDelegate = null;
                processingInfo.DeserializePrimitiveDelegate = null;
                processingInfo.SerializeObjectDelegate = serializeObjectDelegate;
                processingInfo.DeserializeObjectDelegate = deserializeObjectDelegate;
                processingInfo.SerializeFileDelegate = null;
                processingInfo.DeserializeFileDelegate = null;
                processingInfo.SerializeFolderDelegate = null;
                processingInfo.DeserializeFolderDelegate = null;

                return processingInfo;
            }

            public static ProcessingInfo Create_File(TypeInfo typeInfo, SerializeFileDelegate serializeFileDelegate, DeserializeFileDelegate deserializeFileDelegate)
            {
                ProcessingInfo processingInfo = new ProcessingInfo();

                processingInfo.TypeInfo = typeInfo;
                processingInfo.SerializePrimitiveDelegate = null;
                processingInfo.DeserializePrimitiveDelegate = null;
                processingInfo.SerializeObjectDelegate = null;
                processingInfo.DeserializeObjectDelegate = null;
                processingInfo.SerializeFileDelegate = serializeFileDelegate;
                processingInfo.DeserializeFileDelegate = deserializeFileDelegate;
                processingInfo.SerializeFolderDelegate = null;
                processingInfo.DeserializeFolderDelegate = null;

                return processingInfo;
            }

            public static ProcessingInfo Create_Folder(TypeInfo typeInfo, SerializeFolderDelegate serializeFolderDelegate, DeserializeFolderDelegate deserializeFolderDelegate)
            {
                ProcessingInfo processingInfo = new ProcessingInfo();

                processingInfo.TypeInfo = typeInfo;
                processingInfo.SerializePrimitiveDelegate = null;
                processingInfo.DeserializePrimitiveDelegate = null;
                processingInfo.SerializeObjectDelegate = null;
                processingInfo.DeserializeObjectDelegate = null;
                processingInfo.SerializeFileDelegate = null;
                processingInfo.DeserializeFileDelegate = null;
                processingInfo.SerializeFolderDelegate = serializeFolderDelegate;
                processingInfo.DeserializeFolderDelegate = deserializeFolderDelegate;

                return processingInfo;
            }
            #endregion
        }

        public sealed class PostProcessingInfo
        {
            #region Properties
            public TypeInfo TypeInfo { get; private set; }

            public Dictionary<Type, SerializePrimitiveDelegate> UtilizedSerializePrimitiveDelegates { get; private set; }
            public Dictionary<Type, DeserializePrimitiveDelegate> UtilizedDeserializePrimitiveDelegates { get; private set; }
            public Dictionary<Type, SerializeObjectDelegate> UtilizedSerializeObjectDelegates { get; private set; }
            public Dictionary<Type, DeserializeObjectDelegate> UtilizedDeserializeObjectDelegates { get; private set; }
            public Dictionary<Type, SerializeFileDelegate> UtilizedSerializeFileDelegates { get; private set; }
            public Dictionary<Type, DeserializeFileDelegate> UtilizedDeserializeFileDelegates { get; private set; }
            public Dictionary<Type, SerializeFolderDelegate> UtilizedSerializeFolderDelegates { get; private set; }
            public Dictionary<Type, DeserializeFolderDelegate> UtilizedDeserializeFolderDelegates { get; private set; }
            #endregion

            #region Static Methods
            public static PostProcessingInfo Create_Primitive(TypeInfo typeInfo)
            {
                PostProcessingInfo postProcessingInfo = new PostProcessingInfo();

                postProcessingInfo.UtilizedSerializePrimitiveDelegates = null;
                postProcessingInfo.UtilizedDeserializePrimitiveDelegates = null;
                postProcessingInfo.UtilizedSerializeObjectDelegates = null;
                postProcessingInfo.UtilizedDeserializeObjectDelegates = null;
                postProcessingInfo.UtilizedSerializeFileDelegates = null;
                postProcessingInfo.UtilizedDeserializeFileDelegates = null;
                postProcessingInfo.UtilizedSerializeFolderDelegates = null;
                postProcessingInfo.UtilizedDeserializeFolderDelegates = null;

                return postProcessingInfo;
            }

            public static PostProcessingInfo Create_Object(TypeInfo typeInfo, Dictionary<Type, SerializePrimitiveDelegate> utilizedSerializePrimitiveDelegates, Dictionary<Type, DeserializePrimitiveDelegate> utilizedDeserializePrimitiveDelegates, Dictionary<Type, SerializeObjectDelegate> utilizedSerializeObjectDelegates, Dictionary<Type, DeserializeObjectDelegate> utilizedDeserializeObjectDelegates)
            {
                PostProcessingInfo postProcessingInfo = new PostProcessingInfo();

                postProcessingInfo.UtilizedSerializePrimitiveDelegates = utilizedSerializePrimitiveDelegates;
                postProcessingInfo.UtilizedDeserializePrimitiveDelegates = utilizedDeserializePrimitiveDelegates;
                postProcessingInfo.UtilizedSerializeObjectDelegates = utilizedSerializeObjectDelegates;
                postProcessingInfo.UtilizedDeserializeObjectDelegates = utilizedDeserializeObjectDelegates;
                postProcessingInfo.UtilizedSerializeFileDelegates = null;
                postProcessingInfo.UtilizedDeserializeFileDelegates = null;
                postProcessingInfo.UtilizedSerializeFolderDelegates = null;
                postProcessingInfo.UtilizedDeserializeFolderDelegates = null;

                return postProcessingInfo;
            }

            public static PostProcessingInfo Create_File(TypeInfo typeInfo, Dictionary<Type, SerializeObjectDelegate> utilizedSerializeObjectDelegates, Dictionary<Type, DeserializeObjectDelegate> utilizedDeserializeObjectDelegates)
            {
                PostProcessingInfo postProcessingInfo = new PostProcessingInfo();

                postProcessingInfo.UtilizedSerializePrimitiveDelegates = null;
                postProcessingInfo.UtilizedDeserializePrimitiveDelegates = null;
                postProcessingInfo.UtilizedSerializeObjectDelegates = utilizedSerializeObjectDelegates;
                postProcessingInfo.UtilizedDeserializeObjectDelegates = utilizedDeserializeObjectDelegates;
                postProcessingInfo.UtilizedSerializeFileDelegates = null;
                postProcessingInfo.UtilizedDeserializeFileDelegates = null;
                postProcessingInfo.UtilizedSerializeFolderDelegates = null;
                postProcessingInfo.UtilizedDeserializeFolderDelegates = null;

                return postProcessingInfo;
            }

            public static PostProcessingInfo Create_Folder(TypeInfo typeInfo, Dictionary<Type, SerializeFileDelegate> utilizedSerializeFileDelegates, Dictionary<Type, DeserializeFileDelegate> utilizedDeserializeFileDelegates, Dictionary<Type, SerializeFolderDelegate> utilizedSerializeFolderDelegates, Dictionary<Type, DeserializeFolderDelegate> utilizedDeserializeFolderDelegates)
            {
                PostProcessingInfo postProcessingInfo = new PostProcessingInfo();

                postProcessingInfo.UtilizedSerializePrimitiveDelegates = null;
                postProcessingInfo.UtilizedDeserializePrimitiveDelegates = null;
                postProcessingInfo.UtilizedSerializeObjectDelegates = null;
                postProcessingInfo.UtilizedDeserializeObjectDelegates = null;
                postProcessingInfo.UtilizedSerializeFileDelegates = utilizedSerializeFileDelegates;
                postProcessingInfo.UtilizedDeserializeFileDelegates = utilizedDeserializeFileDelegates;
                postProcessingInfo.UtilizedSerializeFolderDelegates = utilizedSerializeFolderDelegates;
                postProcessingInfo.UtilizedDeserializeFolderDelegates = utilizedDeserializeFolderDelegates;

                return postProcessingInfo;
            }
            #endregion
        }
        #endregion

        #region Properties
        public Type Type { get; private set; }
        public SerializabilityValidationStage SerializabilityValidationStage { get; private set; }
        public PreAnalysisInfo PreAnalysisInformation { get; private set; }
        public AnalysisInfo AnalysisInformation { get; private set; }
        public PostAnalysisInfo PostAnalysisInformation { get; private set; }
        public PreProcessingInfo PreProcessingInformation { get; private set; }
        public ProcessingInfo ProcessingInformation { get; private set; }
        public PostProcessingInfo PostProcessingInformation { get; private set; }
        #endregion

        #region Constructors
        public TypeInfo(Type type)
        {
            Type = type;
            SerializabilityValidationStage = SerializabilityValidationStage.Unvalidated;
        }
        #endregion

        #region Methods
        public void Invalidate()
        {
            if (SerializabilityValidationStage == SerializabilityValidationStage.Validated)
            {
                throw new InvalidOperationException("TypeInfo has already been validated!");
            }
            
            SerializabilityValidationStage = SerializabilityValidationStage.Invalidated;
        }

        public void Validate()
        {
            if (SerializabilityValidationStage == SerializabilityValidationStage.Invalidated)
            {
                throw new InvalidOperationException("TypeInfo has already been invalidated!");
            }
            if (SerializabilityValidationStage != SerializabilityValidationStage.PostProcessed)
            {
                throw new InvalidOperationException("Validation can only be performed wheh the type info is post-processed!");
            }

            SerializabilityValidationStage = SerializabilityValidationStage.Validated;
        }

        public void PreAnalyze(PreAnalysisInfo preAnalysisInformation)
        {
            if (SerializabilityValidationStage != SerializabilityValidationStage.Unvalidated)
            {
                throw new InvalidOperationException("Pre-Analysis must be performed when the type info is unvalidated!");
            }
            if (SerializabilityValidationStage == SerializabilityValidationStage.Invalidated)
            {
                throw new InvalidOperationException("TypeInfo has already been invalidated!");
            }

            PreAnalysisInformation = preAnalysisInformation;
        }

        public void Analyze(AnalysisInfo analysisInformation)
        {
            if (SerializabilityValidationStage != SerializabilityValidationStage.PreAnalyzed)
            {
                throw new InvalidOperationException("Analysis must be performed when the type info is pre-analyzed!!");
            }
            if (SerializabilityValidationStage == SerializabilityValidationStage.Invalidated)
            {
                throw new InvalidOperationException("TypeInfo has already been invalidated!");
            }

            AnalysisInformation = analysisInformation;
        }

        public void PostAnalyze(PostAnalysisInfo postAnalysisInfo)
        {
            if (SerializabilityValidationStage != SerializabilityValidationStage.Analyzed)
            {
                throw new InvalidOperationException("Post-Analysis must be performed when the type info is analyzed!");
            }
            if (SerializabilityValidationStage == SerializabilityValidationStage.Invalidated)
            {
                throw new InvalidOperationException("TypeInfo has already been invalidated!");
            }

            PostAnalysisInformation = postAnalysisInfo;
        }

        public void PreProcess(PreProcessingInfo preProcessingInformation)
        {
            if (SerializabilityValidationStage != SerializabilityValidationStage.PostAnalyzed)
            {
                throw new InvalidOperationException("Pre-Processing must be performed when the type info is post-analyzed!");
            }
            if (SerializabilityValidationStage == SerializabilityValidationStage.Invalidated)
            {
                throw new InvalidOperationException("TypeInfo has already been invalidated!");
            }

            PreProcessingInformation = preProcessingInformation;
        }

        public void Process(ProcessingInfo processingInformation)
        {
            if (SerializabilityValidationStage != SerializabilityValidationStage.PreProcessed)
            {
                throw new InvalidOperationException("Processing must be performed when the type info is pre-processed!");
            }
            if (SerializabilityValidationStage == SerializabilityValidationStage.Invalidated)
            {
                throw new InvalidOperationException("TypeInfo has already been invalidated!");
            }

            ProcessingInformation = processingInformation;
        }

        public void PostProcess(PostProcessingInfo postProcessingInformation)
        {
            if (SerializabilityValidationStage != SerializabilityValidationStage.Processed)
            {
                throw new InvalidOperationException("Post-Processing must be performed when the type info is processed!");
            }
            if (SerializabilityValidationStage == SerializabilityValidationStage.Invalidated)
            {
                throw new InvalidOperationException("TypeInfo has already been invalidated!");
            }

            PostProcessingInformation = postProcessingInformation;
        }
        #endregion
    }
}
