using System;
using System.Collections.Generic;
using System.Xml.Linq;

namespace LooCast.System.Serialization
{
    public sealed class GenericObjectTypeInfo : ObjectTypeInfo
    {
        #region Enums
        public enum ValidationStage
        {
            Unvalidated,
            PreAnalyzed,
            Analyzed,
            Processed,
            Invalidated,
            Validated
        }
        #endregion

        #region Properties
        public ValidationStage Validation { get; private set; }
        public PreAnalysisInfo PreAnalysisInformation { get; private set; }
        public AnalysisInfo AnalysisInformation { get; private set; }
        public ProcessingInfo ProcessingInformation { get; private set; }
        #endregion

        #region Constructors
        public GenericObjectTypeInfo()
        {
            Validation = ValidationStage.Unvalidated;
            PreAnalysisInformation = null;
            AnalysisInformation = null;
            ProcessingInformation = null;
        }
        #endregion

        #region Methods
        public void Invalidate()
        {
            if (Validation == ValidationStage.Validated)
            {
                throw new InvalidOperationException("Generic object type info has already been validated!");
            }

            Validation = ValidationStage.Invalidated;
        }

        public void Validate()
        {
            if (Validation == ValidationStage.Invalidated)
            {
                throw new InvalidOperationException("Generic object type info has already been invalidated!");
            }
            if (Validation != ValidationStage.Processed)
            {
                throw new InvalidOperationException("Validation can only be performed when the non-generic object type info is processed!");
            }

            Validation = ValidationStage.Validated;
        }

        public void PreAnalyze(PreAnalysisInfo preAnalysisInformation)
        {
            if (Validation == ValidationStage.Invalidated)
            {
                throw new InvalidOperationException("Generic object type info has already been invalidated!");
            }
            if (Validation != ValidationStage.Unvalidated)
            {
                throw new InvalidOperationException("Pre-Analysis can only be performed when the non-generic object type info is unvalidated!");
            }

            PreAnalysisInformation = preAnalysisInformation;
        }

        public void Analyze(AnalysisInfo analysisInformation)
        {
            if (Validation == ValidationStage.Invalidated)
            {
                throw new InvalidOperationException("Generic object type info has already been invalidated!");
            }
            if (Validation != ValidationStage.PreAnalyzed)
            {
                throw new InvalidOperationException("Analysis can only be performed when the non-generic object type info is pre-analyzed!!");
            }

            AnalysisInformation = analysisInformation;
        }

        public void Process(ProcessingInfo processingInformation)
        {
            if (Validation == ValidationStage.Invalidated)
            {
                throw new InvalidOperationException("Generic object type info has already been invalidated!");
            }
            if (Validation != ValidationStage.Analyzed)
            {
                throw new InvalidOperationException("Processing can only be performed when the non-generic object type info is analyzed!");
            }

            ProcessingInformation = processingInformation;
        }
        #endregion
    }
}
