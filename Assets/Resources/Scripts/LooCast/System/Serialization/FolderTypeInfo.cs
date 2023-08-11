using System;
using System.Collections.Generic;
using System.IO;
using System.Reflection;

namespace LooCast.System.Serialization
{
    public sealed class FolderTypeInfo
    {
        #region Delegates
        public delegate void Serialize(string folderName, string parentFolderPath, object folder, out DirectoryInfo serializedFolder);
        public delegate void Deserialize(DirectoryInfo serializedFolder, out object folder);
        #endregion

        #region Enums
        public enum ValidationStage
        {
            Unvalidated,
            PreAnalyzed,
            Analyzed,
            PreProcessed,
            Processed,
            Invalidated,
            Validated
        }
        #endregion

        #region Classes
        public sealed class PreAnalysisInfo
        {
            #region Properties
            public PropertyInfo[] Properties { get; private set; }
            public FieldInfo[] Fields { get; private set; }
            #endregion

            #region Constructors
            public PreAnalysisInfo(PropertyInfo[] properties, FieldInfo[] fields)
            {
                Properties = properties;
                Fields = fields;
            }
            #endregion
        }

        public sealed class AnalysisInfo
        {
            #region Properties
            public HashSet<TypeInfo> FileTypeDependencies { get; private set; }
            public HashSet<TypeInfo> FolderTypeDependencies { get; private set; }
            #endregion

            #region Constructors
            public AnalysisInfo(HashSet<TypeInfo> fileTypeDependencies, HashSet<TypeInfo> folderTypeDependencies)
            {
                FileTypeDependencies = fileTypeDependencies;
                FolderTypeDependencies = folderTypeDependencies;
            }
            #endregion
        }

        public sealed class PreProcessingInfo
        {
            #region Properties
            public bool OverrideSerialization { get; private set; }
            public bool OverrideDeserialization { get; private set; }
            #endregion

            #region Constructors
            public PreProcessingInfo(bool overrideSerialization, bool overrideDeserialization)
            {
                OverrideSerialization = overrideSerialization;
                OverrideDeserialization = overrideDeserialization;
            }
            #endregion
        }

        public sealed class ProcessingInfo
        {
            #region Properties
            public Serialize SerializeDelegate { get; private set; }
            public Deserialize DeserializeDelegate { get; private set; }
            #endregion

            #region Constructors
            public ProcessingInfo(Serialize serializeDelegate, Deserialize deserializeDelegate)
            {
                SerializeDelegate = serializeDelegate;
                DeserializeDelegate = deserializeDelegate;
            }
            #endregion
        }
        #endregion

        #region Properties
        public ValidationStage Validation { get; private set; }
        public PreAnalysisInfo PreAnalysisInformation { get; private set; }
        public AnalysisInfo AnalysisInformation { get; private set; }
        public PreProcessingInfo PreProcessingInformation { get; private set; }
        public ProcessingInfo ProcessingInformation { get; private set; }
        #endregion

        #region Constructors
        public FolderTypeInfo()
        {
            Validation = ValidationStage.Unvalidated;
            PreAnalysisInformation = null;
            AnalysisInformation = null;
            PreProcessingInformation = null;
            ProcessingInformation = null;
        }
        #endregion

        #region Methods
        public void Invalidate()
        {
            if (Validation == ValidationStage.Validated)
            {
                throw new InvalidOperationException("Folder type info has already been validated!");
            }

            Validation = ValidationStage.Invalidated;
        }

        public void Validate()
        {
            if (Validation == ValidationStage.Invalidated)
            {
                throw new InvalidOperationException("Folder type info has already been invalidated!");
            }
            if (Validation != ValidationStage.Processed)
            {
                throw new InvalidOperationException("Validation can only be performed when the folder type info is processed!");
            }

            Validation = ValidationStage.Validated;
        }

        public void PreAnalyze(PreAnalysisInfo preAnalysisInformation)
        {
            if (Validation == ValidationStage.Invalidated)
            {
                throw new InvalidOperationException("Folder type info has already been invalidated!");
            }
            if (Validation != ValidationStage.Unvalidated)
            {
                throw new InvalidOperationException("Pre-Analysis can only be performed when the folder type info is unvalidated!");
            }

            PreAnalysisInformation = preAnalysisInformation;
        }

        public void Analyze(AnalysisInfo analysisInformation)
        {
            if (Validation == ValidationStage.Invalidated)
            {
                throw new InvalidOperationException("Folder type info has already been invalidated!");
            }
            if (Validation != ValidationStage.PreAnalyzed)
            {
                throw new InvalidOperationException("Analysis can only be performed when the folder type info is pre-analyzed!!");
            }

            AnalysisInformation = analysisInformation;
        }

        public void PreProcess(PreProcessingInfo preProcessingInformation)
        {
            if (Validation == ValidationStage.Invalidated)
            {
                throw new InvalidOperationException("Folder type info has already been invalidated!");
            }
            if (Validation != ValidationStage.Analyzed)
            {
                throw new InvalidOperationException("Pre-Processing can only be performed when the folder type info is analyzed!");
            }

            PreProcessingInformation = preProcessingInformation;
        }

        public void Process(ProcessingInfo processingInformation)
        {
            if (Validation == ValidationStage.Invalidated)
            {
                throw new InvalidOperationException("Folder type info has already been invalidated!");
            }
            if (Validation != ValidationStage.PreProcessed)
            {
                throw new InvalidOperationException("Processing can only be performed when the folder type info is pre-processed!");
            }

            ProcessingInformation = processingInformation;
        }
        #endregion
    }
}
