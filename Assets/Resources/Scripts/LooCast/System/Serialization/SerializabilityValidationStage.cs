using System;

namespace LooCast.System.Serialization
{
    public enum SerializabilityValidationStage
    {
        Unvalidated,
        PreAnalyzed,
        Analyzed,
        PostAnalyzed,
        PreProcessed,
        Processed,
        PostProcessed,
        Invalidated,
        Validated
    }
}
