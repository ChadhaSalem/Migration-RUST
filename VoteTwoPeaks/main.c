#include <stdio.h>
#include<stdint.h>
#include<stdbool.h>



float VoteTwoPeaks(float* pHeartRateEst_max1, float* pConfidenceMetric_max1, float* pHeartRateEst_max2, float* pConfidenceMetric_max2, uint16_t rangeBinIndexShifted, uint8_t thresh, float* pScore)
{
    float votes[8] = {0},maxVotes = 0;
    uint16_t index, indexInnerLoop, qualified = 0;
    for(index = rangeBinIndexShifted-1; index <= rangeBinIndexShifted+2; ++index)
    {
        votes[index - (rangeBinIndexShifted-1)] = pConfidenceMetric_max1[index];
        votes[index + 4 - (rangeBinIndexShifted-1)] = pConfidenceMetric_max2[index];
    }

    for(index = rangeBinIndexShifted-1; index <= rangeBinIndexShifted+2; ++index)
    {
        for(indexInnerLoop = rangeBinIndexShifted-1; indexInnerLoop <= rangeBinIndexShifted+2; ++indexInnerLoop)
        {
            //vote for max1
            if(fabs(pHeartRateEst_max1[index] - pHeartRateEst_max1[indexInnerLoop]) < thresh)
            {
                votes[index - (rangeBinIndexShifted-1)] += pConfidenceMetric_max1[indexInnerLoop];
            }
            if(fabs(pHeartRateEst_max1[index] - pHeartRateEst_max2[indexInnerLoop]) < thresh)
            {
                votes[index - (rangeBinIndexShifted-1)] += pConfidenceMetric_max2[indexInnerLoop];
            }
            //vote for max2
            if(fabs(pHeartRateEst_max2[index] - pHeartRateEst_max1[indexInnerLoop]) < thresh)
            {
                votes[index - (rangeBinIndexShifted-1) + 4] += pConfidenceMetric_max1[indexInnerLoop];
            }
            if(fabs(pHeartRateEst_max2[index] - pHeartRateEst_max2[indexInnerLoop]) < thresh)
            {
                votes[index - (rangeBinIndexShifted-1) + 4] += pConfidenceMetric_max2[indexInnerLoop];
            }
        }
        if(votes[index - (rangeBinIndexShifted-1) + 4] > maxVotes)
        {
            qualified = index + 4;
            maxVotes = votes[index - (rangeBinIndexShifted-1) + 4];
        }

        if(votes[index - (rangeBinIndexShifted-1)] > maxVotes)
        {
            qualified = index;
            maxVotes = votes[index - (rangeBinIndexShifted-1)];
        }
    }
    *pScore = maxVotes;
    if(qualified - (rangeBinIndexShifted - 1) < 4)
        return pHeartRateEst_max1[qualified];
    else
        return pHeartRateEst_max2[qualified - 4];
}