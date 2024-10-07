#include <stdio.h>
#include<stdint.h>
#include<stdbool.h>

float Vote(float* pHeartRateEst, float* pConfidenceMetric, uint16_t rangeBinIndexShifted, uint8_t thresh)
{
    float votes[4] = {0},maxVotes = 0;
    uint16_t index, indexInnerLoop, qualified = 0;

    for(index = rangeBinIndexShifted-1; index <= rangeBinIndexShifted+2; ++index)
    {
        for(indexInnerLoop = rangeBinIndexShifted-1; indexInnerLoop <= rangeBinIndexShifted+2; ++indexInnerLoop)
        {
            if(fabs(pHeartRateEst[index] - pHeartRateEst[indexInnerLoop]) < thresh)
            {
                votes[index - (rangeBinIndexShifted-1)] += pConfidenceMetric[indexInnerLoop];
            }
        }
        if(votes[index - (rangeBinIndexShifted-1)] > maxVotes)
        {
            qualified = index;
            maxVotes = votes[index - (rangeBinIndexShifted-1)];
        }
    }

    return pHeartRateEst[qualified];
}
